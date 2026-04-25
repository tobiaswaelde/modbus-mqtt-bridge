use std::{collections::HashSet, fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// Top-level runtime configuration loaded from YAML or JSON.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub mqtt: MqttConfig,
    pub sources: Vec<SourceConfig>,
    #[serde(default)]
    pub logging: LoggingConfig,
}

impl AppConfig {
    /// Loads the configuration file from disk.
    ///
    /// Supported file extensions:
    /// - `.yml` / `.yaml`
    /// - `.json`
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;

        // The extension determines which parser to use so we can support both YAML and JSON.
        let ext = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        let config = match ext.as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&raw)
                .with_context(|| format!("failed to parse config file {}", path.display()))?,
            "json" => serde_json::from_str(&raw)
                .with_context(|| format!("failed to parse config file {}", path.display()))?,
            _ => bail!("unsupported config extension for {}", path.display()),
        };

        Ok(config)
    }

    /// Validates cross-field constraints that cannot be expressed via serde alone.
    pub fn validate(&self) -> Result<()> {
        if self.mqtt.host.trim().is_empty() {
            bail!("mqtt.host must not be empty");
        }
        if self.mqtt.port == 0 {
            bail!("mqtt.port must be greater than 0");
        }
        if self.mqtt.client_id.trim().is_empty() {
            bail!("mqtt.client_id must not be empty");
        }
        if self.mqtt.base_topic.trim().is_empty() {
            bail!("mqtt.base_topic must not be empty");
        }
        if topic_has_wildcards(&self.mqtt.base_topic) {
            bail!("mqtt.base_topic must not contain MQTT wildcards '+' or '#'");
        }
        if self.mqtt.keep_alive_secs == 0 {
            bail!("mqtt.keep_alive_secs must be greater than 0");
        }

        if self.sources.is_empty() {
            bail!("config must contain at least one source");
        }

        let mut source_ids = HashSet::new();
        for source in &self.sources {
            if source.id.trim().is_empty() {
                bail!("source id must not be empty");
            }
            if !source_ids.insert(source.id.as_str()) {
                bail!("duplicate source id '{}'", source.id);
            }
            if source.host.trim().is_empty() {
                bail!("source '{}' host must not be empty", source.id);
            }
            if source.port == 0 {
                bail!("source '{}' port must be greater than 0", source.id);
            }
            if source.poll_interval_ms == 0 {
                bail!(
                    "source '{}' poll_interval_ms must be greater than 0",
                    source.id
                );
            }
            if source.request_timeout_ms == 0 {
                bail!(
                    "source '{}' request_timeout_ms must be greater than 0",
                    source.id
                );
            }
            if source.modbus_retries > 0 && source.modbus_retry_backoff_ms == 0 {
                bail!(
                    "source '{}' modbus_retry_backoff_ms must be greater than 0 when modbus_retries > 0",
                    source.id
                );
            }

            // A source without points is almost always a configuration mistake.
            if source.points.is_empty() {
                bail!("source '{}' must define at least one point", source.id);
            }

            let mut point_topics = HashSet::new();
            for point in &source.points {
                if point.name.trim().is_empty() {
                    bail!("source '{}' has a point with empty name", source.id);
                }
                if point.topic.trim().is_empty() {
                    bail!(
                        "source '{}' point '{}' has an empty topic",
                        source.id,
                        point.name
                    );
                }
                if topic_has_wildcards(&point.topic) {
                    bail!(
                        "source '{}' point '{}' topic must not contain MQTT wildcards '+' or '#'",
                        source.id,
                        point.name
                    );
                }
                if point.topic.trim_matches('/').ends_with("/set") {
                    bail!(
                        "source '{}' point '{}' topic must not end with '/set'",
                        source.id,
                        point.name
                    );
                }
                if !point_topics.insert(point.topic.as_str()) {
                    bail!(
                        "source '{}' has duplicate point topic '{}'",
                        source.id,
                        point.topic
                    );
                }
            }
        }

        Ok(())
    }
}

fn topic_has_wildcards(topic: &str) -> bool {
    topic.contains('+') || topic.contains('#')
}

/// MQTT connection and topic settings.
#[derive(Debug, Clone, Deserialize)]
pub struct MqttConfig {
    pub host: String,
    #[serde(default = "default_mqtt_port")]
    pub port: u16,
    #[serde(default = "default_client_id")]
    pub client_id: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default = "default_base_topic")]
    pub base_topic: String,
    #[serde(default = "default_keep_alive_secs")]
    pub keep_alive_secs: u64,
    #[serde(default = "default_reconnect_delay_secs")]
    pub reconnect_delay_secs: u64,
}

/// One Modbus TCP endpoint with its polling schedule and points.
#[derive(Debug, Clone, Deserialize)]
pub struct SourceConfig {
    // Unique source identifier used in MQTT topic paths.
    pub id: String,
    pub host: String,
    #[serde(default = "default_modbus_port")]
    pub port: u16,
    pub unit_id: u8,
    #[serde(default = "default_poll_interval_ms")]
    pub poll_interval_ms: u64,
    #[serde(default = "default_request_timeout_ms")]
    pub request_timeout_ms: u64,
    #[serde(default = "default_modbus_retries")]
    pub modbus_retries: u32,
    #[serde(default = "default_modbus_retry_backoff_ms")]
    pub modbus_retry_backoff_ms: u64,
    #[serde(default)]
    pub points: Vec<PointConfig>,
}

/// Mapping definition for one telemetry/control point.
#[derive(Debug, Clone, Deserialize)]
pub struct PointConfig {
    // Human-readable display name (used for logs and diagnostics).
    pub name: String,
    // Relative MQTT path segment under <base_topic>/<source_id>/...
    pub topic: String,
    // Modbus start address for the point.
    pub address: u16,
    pub kind: RegisterKind,
    pub data_type: DataType,
    #[serde(default)]
    pub access: Access,
    #[serde(default)]
    pub count: Option<u16>,
    #[serde(default)]
    pub encoding: Encoding,
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default)]
    pub offset: Option<f64>,
    #[serde(default)]
    pub retain: Option<bool>,
}

/// Supported Modbus function groups for read/write operations.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegisterKind {
    Coil,
    DiscreteInput,
    Holding,
    Input,
}

/// Logical data representation used for decode/encode.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    Bool,
    U16,
    I16,
    U32,
    I32,
    F32,
    String,
    RawU16,
}

/// Byte order inside a 16-bit Modbus register.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ByteOrder {
    #[default]
    Big,
    Little,
}

/// Word order across multiple 16-bit registers.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WordOrder {
    #[default]
    Big,
    Little,
}

/// Combined byte/word ordering for multi-register values.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct Encoding {
    #[serde(default)]
    pub byte_order: ByteOrder,
    #[serde(default)]
    pub word_order: WordOrder,
}

impl Default for Encoding {
    fn default() -> Self {
        // Industry-default layout for most PLC/energy devices.
        Self {
            byte_order: ByteOrder::Big,
            word_order: WordOrder::Big,
        }
    }
}

/// Read/write access policy for a point.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    ReadOnly,
    WriteOnly,
    #[default]
    ReadWrite,
}

impl Access {
    /// Returns `true` when this access mode permits reading.
    pub fn can_read(self) -> bool {
        matches!(self, Self::ReadOnly | Self::ReadWrite)
    }

    /// Returns `true` when this access mode permits writing.
    pub fn can_write(self) -> bool {
        matches!(self, Self::WriteOnly | Self::ReadWrite)
    }
}

/// Logging output configuration.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub json: bool,
}

fn default_mqtt_port() -> u16 {
    1883
}

fn default_client_id() -> String {
    "modbus-mqtt-bridge".to_string()
}

fn default_base_topic() -> String {
    "modbus".to_string()
}

fn default_keep_alive_secs() -> u64 {
    30
}

fn default_reconnect_delay_secs() -> u64 {
    5
}

fn default_modbus_port() -> u16 {
    502
}

fn default_poll_interval_ms() -> u64 {
    1_000
}

fn default_request_timeout_ms() -> u64 {
    3_000
}

fn default_modbus_retries() -> u32 {
    0
}

fn default_modbus_retry_backoff_ms() -> u64 {
    250
}

fn default_log_level() -> String {
    "info".to_string()
}
