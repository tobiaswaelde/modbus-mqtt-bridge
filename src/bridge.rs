use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Context, Result, anyhow, bail};
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, Publish, QoS};
use serde_json::Value;
use tokio::{net::TcpStream, sync::mpsc, time};
use tokio_modbus::{client::tcp, prelude::*};
use tracing::{debug, error, info, warn};

use crate::{
    config::{AppConfig, PointConfig, RegisterKind, SourceConfig},
    modbus_codec::{EncodedWrite, decode_point, encode_write_payload, register_count},
};

// MQTT write requests are dispatched onto an internal channel so the event loop stays responsive.
#[derive(Debug, Clone)]
pub struct RpcCommand {
    pub source_id: String,
    pub point_topic: String,
    pub payload: Value,
}

pub async fn run(config: AppConfig) -> Result<()> {
    let mqtt_options = mqtt_options(&config);
    let (mqtt_client, mut event_loop) = AsyncClient::new(mqtt_options, 100);
    let mqtt_client = Arc::new(mqtt_client);

    let (rpc_tx, mut rpc_rx) = mpsc::channel::<RpcCommand>(100);
    subscribe_to_rpc_topics(&mqtt_client, &config).await?;

    let event_client = Arc::clone(&mqtt_client);
    let event_config = config.clone();
    tokio::spawn(async move {
        if let Err(error) = drive_mqtt_event_loop(&mut event_loop, rpc_tx, &event_config).await {
            error!(error = ?error, "mqtt event loop stopped");
        }
        drop(event_client);
    });

    for source in config.sources.iter().cloned() {
        let client = Arc::clone(&mqtt_client);
        let base_topic = config.mqtt.base_topic.clone();
        tokio::spawn(async move {
            poll_source_loop(client, &base_topic, source).await;
        });
    }

    while let Some(command) = rpc_rx.recv().await {
        let source = config
            .sources
            .iter()
            .find(|source| source.id == command.source_id)
            .cloned()
            .ok_or_else(|| anyhow!("received rpc for unknown source '{}'", command.source_id))?;

        if let Err(error) =
            handle_rpc_command(&mqtt_client, &config.mqtt.base_topic, &source, command).await
        {
            error!(source = source.id, error = ?error, "rpc command failed");
        }
    }

    bail!("mqtt command channel closed")
}

fn mqtt_options(config: &AppConfig) -> MqttOptions {
    let mut options = MqttOptions::new(
        config.mqtt.client_id.clone(),
        config.mqtt.host.clone(),
        config.mqtt.port,
    );
    options.set_keep_alive(Duration::from_secs(config.mqtt.keep_alive_secs));

    if let Some(username) = &config.mqtt.username {
        options.set_credentials(username, config.mqtt.password.clone().unwrap_or_default());
    }

    options
}

async fn subscribe_to_rpc_topics(client: &AsyncClient, config: &AppConfig) -> Result<()> {
    for source in &config.sources {
        for point in &source.points {
            if !point.access.can_write() {
                continue;
            }
            let topic = set_topic(&config.mqtt.base_topic, &source.id, &point.topic);
            client.subscribe(topic.clone(), QoS::AtLeastOnce).await?;
            info!(
                source = source.id,
                point = point.name,
                topic,
                "subscribed to set topic"
            );
        }
    }
    Ok(())
}

async fn drive_mqtt_event_loop(
    event_loop: &mut EventLoop,
    rpc_tx: mpsc::Sender<RpcCommand>,
    config: &AppConfig,
) -> Result<()> {
    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                if let Some(command) = decode_rpc_publish(config, publish)? {
                    rpc_tx.send(command).await?;
                }
            }
            Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                info!("connected to mqtt broker");
            }
            Ok(Event::Outgoing(_)) | Ok(Event::Incoming(_)) => {}
            Err(error) => {
                warn!(error = ?error, delay_secs = config.mqtt.reconnect_delay_secs, "mqtt loop error");
                time::sleep(Duration::from_secs(config.mqtt.reconnect_delay_secs)).await;
            }
        }
    }
}

fn decode_rpc_publish(config: &AppConfig, publish: Publish) -> Result<Option<RpcCommand>> {
    let prefix = format!("{}/", config.mqtt.base_topic.trim_end_matches('/'));
    if !publish.topic.starts_with(&prefix) || !publish.topic.ends_with("/set") {
        return Ok(None);
    }

    // Topics are shaped like `<base>/<source>/<point>/set`, so split only once after the source id.
    let trimmed = publish
        .topic
        .trim_start_matches(&prefix)
        .trim_end_matches("/set");
    let mut parts = trimmed.splitn(2, '/');
    let source_id = parts
        .next()
        .ok_or_else(|| anyhow!("rpc topic missing source id"))?;
    let point_topic = parts
        .next()
        .ok_or_else(|| anyhow!("rpc topic missing point topic"))?;

    let payload = parse_rpc_payload(&publish.payload)?;

    Ok(Some(RpcCommand {
        source_id: source_id.to_string(),
        point_topic: point_topic.to_string(),
        payload,
    }))
}

fn parse_rpc_payload(bytes: &[u8]) -> Result<Value> {
    let payload: Value = serde_json::from_slice(bytes)
        .with_context(|| "rpc payload must be valid JSON".to_string())?;

    // Accept either a raw JSON value or `{ "value": ... }` for easier MQTT publisher integration.
    if let Some(value) = payload.get("value") {
        Ok(value.clone())
    } else {
        Ok(payload)
    }
}

async fn poll_source_loop(client: Arc<AsyncClient>, base_topic: &str, source: SourceConfig) {
    let interval = Duration::from_millis(source.poll_interval_ms);
    loop {
        let started = time::Instant::now();
        if let Err(error) = poll_source_once(&client, base_topic, &source).await {
            warn!(source = source.id, error = ?error, "poll cycle failed");
        }

        let elapsed = started.elapsed();
        if elapsed < interval {
            time::sleep(interval - elapsed).await;
        }
    }
}

async fn poll_source_once(
    client: &AsyncClient,
    base_topic: &str,
    source: &SourceConfig,
) -> Result<()> {
    for point in &source.points {
        if !point.access.can_read() {
            continue;
        }

        let value = read_point(source, point).await?;
        let topic = state_topic(base_topic, &source.id, &point.topic);

        client
            .publish(
                topic.clone(),
                QoS::AtLeastOnce,
                point.retain.unwrap_or(true),
                serde_json::to_vec(&value)?,
            )
            .await?;
        debug!(
            source = source.id,
            point = point.name,
            topic,
            "published point state"
        );
    }

    Ok(())
}

async fn handle_rpc_command(
    client: &AsyncClient,
    base_topic: &str,
    source: &SourceConfig,
    command: RpcCommand,
) -> Result<()> {
    let point = source
        .points
        .iter()
        .find(|point| point.topic == command.point_topic)
        .ok_or_else(|| anyhow!("unknown point topic '{}'", command.point_topic))?;

    if !point.access.can_write() {
        bail!("point '{}' is not writable", point.name);
    }

    write_point(source, point, &command.payload).await?;
    info!(source = source.id, point = point.name, "write completed");

    if point.access.can_read() {
        let value = read_point(source, point).await?;
        let topic = state_topic(base_topic, &source.id, &point.topic);

        client
            .publish(
                topic,
                QoS::AtLeastOnce,
                point.retain.unwrap_or(true),
                serde_json::to_vec(&value)?,
            )
            .await?;
    }

    Ok(())
}

async fn read_point(source: &SourceConfig, point: &PointConfig) -> Result<Value> {
    let socket_addr: SocketAddr = format!("{}:{}", source.host, source.port).parse()?;
    let timeout = Duration::from_millis(source.request_timeout_ms);

    // Modbus connections are short-lived per request, which keeps reconnect logic simple for containers.
    let result = time::timeout(timeout, async {
        let stream = TcpStream::connect(socket_addr).await?;
        let slave = Slave(source.unit_id);
        let mut ctx = tcp::attach_slave(stream, slave);

        match point.kind {
            RegisterKind::Coil => {
                let response = ctx
                    .read_coils(point.address, register_count(point))
                    .await??;
                Ok::<_, anyhow::Error>(decode_point(point, Some(&response), None)?)
            }
            RegisterKind::DiscreteInput => {
                let response = ctx
                    .read_discrete_inputs(point.address, register_count(point))
                    .await??;
                Ok(decode_point(point, Some(&response), None)?)
            }
            RegisterKind::Holding => {
                let response = ctx
                    .read_holding_registers(point.address, register_count(point))
                    .await??;
                Ok(decode_point(point, None, Some(&response))?)
            }
            RegisterKind::Input => {
                let response = ctx
                    .read_input_registers(point.address, register_count(point))
                    .await??;
                Ok(decode_point(point, None, Some(&response))?)
            }
        }
    })
    .await;

    match result {
        Ok(value) => value,
        Err(_) => bail!(
            "modbus read timed out after {} ms",
            source.request_timeout_ms
        ),
    }
}

async fn write_point(source: &SourceConfig, point: &PointConfig, payload: &Value) -> Result<()> {
    let encoded = encode_write_payload(point, payload)?;
    let socket_addr: SocketAddr = format!("{}:{}", source.host, source.port).parse()?;
    let timeout = Duration::from_millis(source.request_timeout_ms);

    // Reuse the same one-shot connection strategy for writes to avoid stale sockets.
    let result = time::timeout(timeout, async {
        let stream = TcpStream::connect(socket_addr).await?;
        let slave = Slave(source.unit_id);
        let mut ctx = tcp::attach_slave(stream, slave);

        match encoded {
            EncodedWrite::Coil(value) => {
                ctx.write_single_coil(point.address, value).await??;
            }
            EncodedWrite::Registers(registers) => {
                if registers.len() == 1 {
                    ctx.write_single_register(point.address, registers[0])
                        .await??;
                } else {
                    ctx.write_multiple_registers(point.address, &registers)
                        .await??;
                }
            }
        }

        Ok::<_, anyhow::Error>(())
    })
    .await;

    match result {
        Ok(value) => value,
        Err(_) => bail!(
            "modbus write timed out after {} ms",
            source.request_timeout_ms
        ),
    }
}

fn state_topic(base_topic: &str, source_id: &str, point_topic: &str) -> String {
    format!(
        "{}/{}/{}",
        base_topic.trim_end_matches('/'),
        source_id,
        point_topic.trim_start_matches('/')
    )
}

fn set_topic(base_topic: &str, source_id: &str, point_topic: &str) -> String {
    format!(
        "{}/{}/{}/set",
        base_topic.trim_end_matches('/'),
        source_id,
        point_topic.trim_start_matches('/')
    )
}
