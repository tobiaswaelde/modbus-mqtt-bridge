# Deployment

This project supports multiple deployment paths depending on your environment.

## Docker Compose (recommended)

```bash
docker compose up --build -d
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
docker run --rm \
  -v "$(pwd)/config:/app/config:ro" \
  ghcr.io/tobiaswaelde/modbus-mqtt-bridge:latest \
  --config /app/config/config.yml
```

## Bare-metal / VM

Run directly as a binary when containers are not desired:

```bash
cargo build --release
./target/release/modbus-mqtt-bridge --config config/config.yml
```

## GitHub Pages Docs

Documentation is deployed separately from the bridge runtime:

- workflow: `.github/workflows/pages.yml`
- source: `docs/` (VitePress)
- publish output: `docs/.vitepress/dist`

Your runtime deployment and docs deployment are independent, so docs updates do not restart your bridge service.
