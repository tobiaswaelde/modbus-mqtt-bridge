use std::{fs, path::Path, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use tokio::{
    net::TcpStream,
    time::{self, Duration},
};
use tracing_subscriber::{EnvFilter, fmt};

use modbus_mqtt_bridge::{bridge, config::AppConfig};

// Bundled starter config written on first boot for easier local setup.
const EXAMPLE_CONFIG: &str = include_str!("../config/config.yml");

#[derive(Debug, Parser)]
#[command(author, version, about = "Modbus TCP to MQTT bridge")]
struct Cli {
    // Runtime config path (YAML/JSON). Defaults to a repo-local path for simple local startup.
    #[arg(short, long, default_value = "config/config.yml")]
    config: PathBuf,
    // Used by container HEALTHCHECK to validate config parsing and MQTT reachability.
    #[arg(long)]
    healthcheck: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.healthcheck {
        return run_healthcheck(&cli.config).await;
    }

    ensure_config_exists(&cli.config)?;
    let config = AppConfig::load(&cli.config)?;
    config.validate()?;
    init_logging(&config)?;
    bridge::run(config).await
}

fn ensure_config_exists(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    // Bootstraps first startup experience: create missing directories and write a safe template.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config directory {}", parent.display()))?;
    }

    fs::write(path, EXAMPLE_CONFIG)
        .with_context(|| format!("failed to create example config at {}", path.display()))?;

    eprintln!(
        "Created example config at {}. Update it with your environment values and restart if needed.",
        path.display()
    );

    Ok(())
}

async fn run_healthcheck(path: &Path) -> Result<()> {
    let config = AppConfig::load(path)?;
    config.validate()?;

    // A healthy container should at least be able to resolve and open the MQTT TCP socket.
    time::timeout(
        Duration::from_secs(3),
        TcpStream::connect((config.mqtt.host.as_str(), config.mqtt.port)),
    )
    .await
    .context("mqtt healthcheck timed out")?
    .with_context(|| {
        format!(
            "failed to connect to mqtt broker {}:{}",
            config.mqtt.host, config.mqtt.port
        )
    })?;

    Ok(())
}

fn init_logging(config: &AppConfig) -> Result<()> {
    // Allow env override (RUST_LOG), otherwise fall back to config-defined level.
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(config.logging.level.clone()))?;

    if config.logging.json {
        fmt().with_env_filter(filter).json().init();
    } else {
        fmt().with_env_filter(filter).init();
    }

    Ok(())
}
