# Deployment

This page covers practical deployment patterns from local Docker runs to production hosts.

## Recommended default: Docker Compose

```bash
docker compose up --build -d
```

Compose service (from repository):

```yaml
services:
  modbus-mqtt-bridge:
    image: ghcr.io/tobiaswaelde/modbus-mqtt-bridge:latest
    container_name: modbus-mqtt-bridge
    restart: unless-stopped
    volumes:
      - ./config:/app/config:ro
    command: ["--config", "/app/config/config.yml"]
```

Why this is the default:

- reproducible setup across environments
- host-mounted config for easy updates
- built-in healthcheck in container image

## Direct image deployment (GHCR)

```bash
docker pull ghcr.io/tobiaswaelde/modbus-mqtt-bridge:latest
```

```bash
docker run --rm \
  --name modbus-mqtt-bridge \
  -v "$(pwd)/config:/app/config:ro" \
  ghcr.io/tobiaswaelde/modbus-mqtt-bridge:latest \
  --config /app/config/config.yml
```

## Binary deployment (no container runtime)

```bash
cargo build --release
./target/release/modbus-mqtt-bridge --config config/config.yml
```

Use this when you prefer host-level service management (`systemd`, supervisord, etc.).

## Runtime checks

Manual healthcheck command:

```bash
modbus-mqtt-bridge --healthcheck --config config/config.yml
```

Recommended operational checks:

- config file exists and parses (`.yml/.yaml/.json`)
- MQTT DNS/TCP reachability from runtime host
- writable points subscribe to `.../set` as expected

## Upgrade flow

1. Pull new image tag (or build new binary).
2. Keep `config/config.yml` stable and versioned.
3. Restart the service.
4. Confirm healthcheck and MQTT message flow.

## Production notes

- Set `logging.json: true` for structured log pipelines.
- Prefer explicit version tags over `latest` in controlled environments.
- Back up your config repository and change history.
