# Deployment

This project supports multiple deployment paths depending on your environment.

## Container Deployment

::: code-group

```bash [docker compose (recommended)]
docker compose up --build -d
```

```bash [docker run]
docker run --rm \
  --name modbus-mqtt-bridge \
  -v "$(pwd)/config:/app/config:ro" \
  ghcr.io/tobiaswaelde/modbus-mqtt-bridge:latest \
  --config /app/config/config.yml
```

:::

Example `docker-compose.yml`:

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

Why this is usually the best default:

- simple, reproducible setup
- healthcheck built in
- `config/` mounted from host for easy runtime edits

## GHCR Image

The repository publishes a Docker image via GitHub Actions:

- workflow: `.github/workflows/docker.yml`
- image name: `ghcr.io/<owner>/<repo>`

Example pull/run:

```bash
docker pull ghcr.io/tobiaswaelde/modbus-mqtt-bridge:latest
```

## Bare-metal / VM

Run directly as a binary when containers are not desired:

```bash
cargo build --release
./target/release/modbus-mqtt-bridge --config config/config.yml
```
