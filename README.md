# Modbus MQTT Bridge

[![Test](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/test.yml/badge.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/test.yml)
[![Docker](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/docker.yml/badge.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/actions/workflows/docker.yml)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/tobiaswaelde/modbus-mqtt-bridge/blob/master/Cargo.toml)
[![Built with AI](https://img.shields.io/badge/built%20with-AI-0a7ea4.svg)](https://openai.com/)

🔌 A lightweight Rust service that connects Modbus TCP devices to MQTT.

It polls values from Modbus, publishes them to MQTT topics, and listens for MQTT `/set` commands to write values back to writable points.

The checked-in config is intentionally a safe template with placeholder values, so the repository can stay public without exposing real broker credentials or device addresses.

## 🌐 Documentation

Full docs website: **https://tobiaswaelde.github.io/modbus-mqtt-bridge/**

## 📚 Table of Contents

- [✨ Features](#-features)
- [🧠 How It Works](#-how-it-works)
- [🗂️ Topic Layout](#️-topic-layout)
- [⚙️ Configuration](#️-configuration)
- [📝 Example Config](#-example-config)
- [🚀 Run Locally](#-run-locally)
- [🐳 Run With Docker](#-run-with-docker)
- [🌐 Documentation Website](#-documentation-website)
- [🔁 GitHub Actions](#-github-actions)

## ✨ Features

- 🦀 Written in Rust for a small, fast, single-binary deployment
- 🔧 YAML or JSON configuration
- 📝 Auto-creates an example config if the configured file does not exist
- 🏭 Supports multiple Modbus TCP sources
- 📡 Reads coils, discrete inputs, holding registers, and input registers
- 🔢 Supports `bool`, `u16`, `i16`, `u32`, `i32`, `f32`, `string`, and raw register arrays
- 📬 Publishes plain JSON values to MQTT topics
- ✍️ Accepts MQTT `/set` topics for writable points
- 🐳 Docker-friendly defaults

## 🧠 How It Works

1. The bridge reads a config file from `config/config.yml` by default.
2. It connects to your MQTT broker.
3. It polls each configured Modbus point on its interval.
4. It publishes the decoded value to the configured MQTT topic.
5. For writable points, it subscribes to a matching `/set` topic and writes incoming values back to Modbus.

This keeps MQTT consumers simple: they only receive the value they care about, not a larger wrapper payload.

## 🗂️ Topic Layout

For a point with:

- source id: `example-device`
- topic: `telemetry/example_float`
- base topic: `modbus`

the topics become:

- State topic: `modbus/example-device/telemetry/example_float`
- Set topic: `modbus/example-device/telemetry/example_float/set`

Published values are raw JSON:

```json
21.5
```

Set payloads can be either a raw JSON value:

```json
true
```

or an object with a `value` field:

```json
{"value": 18.0}
```

## ⚙️ Configuration

By default the service uses:

```text
config/config.yml
```

If that file does not exist, the bridge creates a starter config automatically at that path.

Each source contains:

- an `id` used in MQTT topics
- the Modbus TCP connection details
- a polling interval
- one or more `points`

Each point defines:

- a human-friendly `name`
- a `topic` suffix
- the Modbus `address`
- the register `kind`
- the `data_type`
- the `access` mode
- optional encoding details such as byte order and word order

## 📝 Example Config

The repository includes this starter config in [config/config.yml](/mnt/projects/tmp/modbus-test/config/config.yml):

```yml
mqtt:
  host: localhost                 # Required. MQTT broker hostname or IP.
  port: 1883                      # Optional. Default: 1883.
  client_id: modbus-mqtt-bridge   # Optional. Default: modbus-mqtt-bridge.
  username:                       # Optional. Leave empty for anonymous access.
  password:                       # Optional. Used when username is set.
  base_topic: modbus              # Optional. Default: modbus.
  keep_alive_secs: 30             # Optional. Default: 30.
  reconnect_delay_secs: 5         # Optional. Default: 5.

logging:
  level: info                     # Optional. Typical values: trace, debug, info, warn, error.
  json: false                     # Optional. true = JSON logs, false = human-readable logs.

sources:
  - id: example-device            # Required. Used in MQTT topics.
    host: 127.0.0.1               # Required. Modbus TCP device hostname or IP.
    port: 502                     # Optional. Default: 502.
    unit_id: 1                    # Required. Modbus slave/unit id.
    poll_interval_ms: 2000        # Optional. Default: 1000.
    request_timeout_ms: 3000      # Optional. Default: 3000.
    points:
      - name: example_float       # Required. Human-friendly point name.
        topic: telemetry/example_float  # Required. Appended under <base_topic>/<source_id>/...
        address: 0                # Required. Modbus start address.
        kind: holding             # Required. One of: coil, discrete_input, holding, input.
        data_type: f32            # Required. One of: bool, u16, i16, u32, i32, f32, string, raw_u16.
        access: read_write        # Optional. One of: read_only, write_only, read_write. Default: read_write.
        encoding:
          byte_order: big         # Optional. One of: big, little. Default: big.
          word_order: big         # Optional. One of: big, little. Default: big.
        retain: true              # Optional. Default: true.
        scale: 0.1                # Optional. Multiplies numeric reads.
        offset: 0.0               # Optional. Added after scaling numeric reads.
      - name: example_coil
        topic: status/example_coil
        address: 1
        kind: coil
        data_type: bool
        access: read_write
      - name: example_label
        topic: info/example_label
        address: 10
        kind: input
        data_type: string
        count: 4                  # Optional. Needed for strings or custom register widths.
        access: read_only
```

Notes:

- `encoding` is mainly relevant for multi-register values like `u32`, `i32`, `f32`, and some string layouts.
- `scale` and `offset` affect values when reading from Modbus. They are useful for sensors that store engineering values in raw units.
- `count` defaults to the natural width of the selected `data_type`, but you can override it for strings and raw register arrays.

## 🚀 Run Locally

Start the bridge with:

```bash
cargo run -- --config config/config.yml
```

Suggested first steps:

1. Update `config/config.yml` with your Modbus device IP and MQTT broker details.
2. Start your broker if it is not already running.
3. Run the bridge.
4. Subscribe to one of the configured MQTT state topics.
5. Publish to a `/set` topic to test writing back to Modbus.

## 🐳 Run With Docker

Build and run with Docker Compose:

```bash
docker compose up --build
```

The compose file mounts the local `config/` directory into the container, so you can edit the config without rebuilding the image.

## 🌐 Documentation Website

- Source files are in `docs/`.
- GitHub Pages deployment is handled by `.github/workflows/pages.yml`.
- After enabling Pages in repository settings, the site is automatically deployed on push to `master`/`main`.

## 🔁 GitHub Actions

- `.github/workflows/test.yml` runs formatting, clippy, tests, and a release build on pushes and pull requests
- `.github/workflows/docker.yml` publishes a Docker image to GHCR using the repository name automatically, for example `ghcr.io/<owner>/<repo>`
- `.github/workflows/pages.yml` publishes the docs website to GitHub Pages
