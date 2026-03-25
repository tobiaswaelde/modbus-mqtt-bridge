# Modbus MQTT Bridge

[![Test](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/test.yml/badge.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/test.yml)
[![Docker](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/docker.yml/badge.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/docker.yml)
[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/blob/master/Cargo.toml)
[![Built with AI](https://img.shields.io/badge/built%20with-AI-0a7ea4.svg)](https://openai.com/)

Rust bridge between Modbus TCP and MQTT.  
It polls Modbus points, publishes state values, and handles writes via matching MQTT `/set` topics.

## 🌐 Documentation

Full docs website: **https://tobiaswaelde.github.io/modbus-mqtt-bridge/**

Recommended entry points:

- [Getting Started](https://tobiaswaelde.github.io/modbus-mqtt-bridge/getting-started)
- [Configuration](https://tobiaswaelde.github.io/modbus-mqtt-bridge/configuration)
- [Deployment](https://tobiaswaelde.github.io/modbus-mqtt-bridge/deployment)
- [Troubleshooting](https://tobiaswaelde.github.io/modbus-mqtt-bridge/troubleshooting)

Project history: [CHANGELOG.md](/mnt/projects/tmp/modbus-test/CHANGELOG.md)

## ✨ Highlights

- Multi-source Modbus TCP polling
- MQTT state + `/set` write contract
- Robust reconnect/retry behavior
- Optional metrics endpoint
- Docker-ready runtime and healthcheck
- GitHub Actions for test, image publish, release, and pages

## 🗂️ Topic Contract

For `base_topic=modbus`, `source_id=example-device`, `topic=telemetry/example_float`:

- State topic: `modbus/example-device/telemetry/example_float`
- Set topic: `modbus/example-device/telemetry/example_float/set`

State payloads are raw JSON values:

```json
21.5
```

Set payloads can be raw or wrapped:

```json
true
```

```json
{"value": 18.0}
```

## ⚙️ Configuration

Default config path:

```text
config/config.yml
```

If it does not exist, the service auto-creates a starter template.

Minimal example:

```yml
mqtt:
  host: localhost
  base_topic: modbus

sources:
  - id: example-device
    host: 127.0.0.1
    unit_id: 1
    points:
      - name: example_float
        topic: telemetry/example_float
        address: 0
        kind: holding
        data_type: f32
```

Full config reference:  
https://tobiaswaelde.github.io/modbus-mqtt-bridge/configuration

## 🚀 Run Locally

```bash
cargo run -- --config config/config.yml
```

## 🐳 Run With Docker

```bash
docker compose up --build
```

Deployment guide (Compose, GHCR, VM, Pages):  
https://tobiaswaelde.github.io/modbus-mqtt-bridge/deployment

## 📚 Docs Development

Docs are built with VitePress from `docs/`:

```bash
cd docs
npm install
npm run dev
```

GitHub Pages publishes `docs/.vitepress/dist` via `.github/workflows/pages.yml`.

## ⚖️ License

Licensed under **GNU GPL v3.0 or later** (`GPL-3.0-or-later`).  
See [LICENSE](/mnt/projects/tmp/modbus-test/LICENSE).
