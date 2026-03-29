<div align="center">
  <img src="docs/public/logo.svg" alt="Modbus MQTT Bridge" width="120" />

  # Modbus MQTT Bridge

  [![Test](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/test.yml/badge.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/test.yml)
  [![Docker](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/docker.yml/badge.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/docker.yml)
  [![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/blob/master/Cargo.toml)
  [![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/blob/master/LICENSE)

  Rust bridge between Modbus TCP devices and MQTT topics.

  [Documentation](https://tobiaswaelde.github.io/modbus-mqtt-bridge/) |
  [Getting Started](https://tobiaswaelde.github.io/modbus-mqtt-bridge/getting-started) |
  [Configuration](https://tobiaswaelde.github.io/modbus-mqtt-bridge/configuration) |
  [Deployment](https://tobiaswaelde.github.io/modbus-mqtt-bridge/deployment)
</div>

## Table of Contents 📚

- [Why this project](#why-this-project)
- [Features](#features)
- [Quick start](#quick-start)
- [Topic contract](#topic-contract)
- [Minimal configuration](#minimal-configuration)
- [Run locally (without Docker)](#run-locally-without-docker)
- [Documentation](#documentation)
- [Project files](#project-files)
- [Changelog](#changelog)
- [License](#license)

## Why this project

`modbus-mqtt-bridge` continuously polls Modbus points and publishes raw JSON values to MQTT state topics. It also accepts writes through matching `/set` topics and forwards those writes back to Modbus.

Use it when you want one stable integration layer between PLCs/inverters/meters and systems like Home Assistant, Node-RED, or custom MQTT consumers.

## Features

- 🔄 Multi-source Modbus TCP polling (independent poll loops per source)
- 📡 Predictable MQTT topic contract (`state` + `/set`)
- 🧠 Typed decode/encode (`bool`, `u16`, `i16`, `u32`, `i32`, `f32`, `string`, `raw_u16`)
- 🛡️ Retry/backoff knobs for Modbus reads/writes
- ❤️ Built-in healthcheck mode for container orchestration
- 🐳 Docker-ready (small `scratch` runtime image)

## Quick start

### 1. 🚀 Run with Docker Compose (recommended)

```bash
git clone https://github.com/tobiaswaelde/modbus-mqtt-bridge.git
cd modbus-mqtt-bridge
cp config/config.example.yml config/config.yml
# edit config/config.yml for your broker and devices

docker compose up --build -d
```

### 2. 🔍 Verify data flow

```bash
mosquitto_sub -h <mqtt-host> -t 'modbus/#' -v
```

Optional write test ✍️:

```bash
mosquitto_pub -h <mqtt-host> -t 'modbus/example-device/status/example_coil/set' -m true
```

## Topic contract

For:

- `base_topic = modbus`
- `source.id = boiler`
- `point.topic = telemetry/temp`

Topics:

- State: `modbus/boiler/telemetry/temp`
- Write: `modbus/boiler/telemetry/temp/set`

Write payloads accepted:

```json
42.5
```

```json
{"value": 42.5}
```

## Minimal configuration

```yaml
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

Full config reference: https://tobiaswaelde.github.io/modbus-mqtt-bridge/configuration

## Run locally (without Docker)

```bash
cargo run -- --config config/config.yml
```

## Documentation

Docs are built with VitePress from `docs/` 📝.

```bash
cd docs
npm install
npm run dev
```

Production docs are published via GitHub Pages from `.github/workflows/pages.yml`.

## Project files

- `src/` runtime and protocol logic
- `config/config.example.yml` starter config template
- `docs/` VitePress documentation source
- `Dockerfile` multi-stage build with `scratch` runtime
- `compose.yml` local deployment template

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Licensed under **GNU GPL v3.0 or later** (`GPL-3.0-or-later`).
See [LICENSE](LICENSE).
