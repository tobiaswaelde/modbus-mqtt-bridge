use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

// Top-level runtime configuration loaded from YAML or JSON.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub mqtt: MqttConfig,
    pub sources: Vec<SourceConfig>,
    #[serde(default)]
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;

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

    pub fn validate(&self) -> Result<()> {
        if self.sources.is_empty() {
            bail!("config must contain at least one source");
        }

        for source in &self.sources {
            // A source without points is almost always a configuration mistake.
            if source.points.is_empty() {
                bail!("source '{}' must define at least one point", source.id);
            }
        }

        Ok(())
    }
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct SourceConfig {
    pub id: String,
    pub host: String,
    #[serde(default = "default_modbus_port")]
    pub port: u16,
    pub unit_id: u8,
    #[serde(default = "default_poll_interval_ms")]
    pub poll_interval_ms: u64,
    #[serde(default = "default_request_timeout_ms")]
    pub request_timeout_ms: u64,
    #[serde(default)]
    pub points: Vec<PointConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointConfig {
    pub name: String,
    pub topic: String,
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

// Mirrors the Modbus function groups the bridge can poll.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegisterKind {
    Coil,
    DiscreteInput,
    Holding,
    Input,
}

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

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ByteOrder {
    #[default]
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WordOrder {
    #[default]
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct Encoding {
    #[serde(default)]
    pub byte_order: ByteOrder,
    #[serde(default)]
    pub word_order: WordOrder,
}

impl Default for Encoding {
    fn default() -> Self {
        Self {
            byte_order: ByteOrder::Big,
            word_order: WordOrder::Big,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    ReadOnly,
    WriteOnly,
    #[default]
    ReadWrite,
}

impl Access {
    pub fn can_read(self) -> bool {
        matches!(self, Self::ReadOnly | Self::ReadWrite)
    }

    pub fn can_write(self) -> bool {
        matches!(self, Self::WriteOnly | Self::ReadWrite)
    }
}

// Logging stays configurable so local development can use text while production can use JSON.
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

fn default_log_level() -> String {
    "info".to_string()
}
