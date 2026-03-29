# Getting Started

This guide gets the bridge from zero to first MQTT payload.

## Prerequisites

- MQTT broker reachable from the bridge (`mosquitto`, EMQX, HiveMQ, ...)
- At least one Modbus TCP device (or simulator)
- One of:
  - Rust toolchain (`cargo`) for local runs
  - Docker + Docker Compose for container runs

## 1. Clone and prepare config

```bash
git clone https://github.com/tobiaswaelde/modbus-mqtt-bridge.git
cd modbus-mqtt-bridge
cp config/config.example.yml config/config.yml
```

Edit `config/config.yml`:

- `mqtt.host` -> your broker host/IP
- `sources[0].host` -> your Modbus endpoint host/IP
- `sources[0].unit_id` -> your device unit/slave ID
- `points[]` -> addresses, register kinds, and data types matching your device map

## 2. Start the bridge

::: code-group

```bash [Local (cargo)]
cargo run -- --config config/config.yml
```

```bash [Docker Compose]
docker compose up --build -d
```

:::

## 3. Verify state publishing

Subscribe to the configured topic namespace:

```bash
mosquitto_sub -h <mqtt-host> -t 'modbus/#' -v
```

You should see payloads on topics like:

- `modbus/<source-id>/<point-topic>`

## 4. Verify write path (`/set`)

Publish a write command to a writable point:

```bash
mosquitto_pub -h <mqtt-host> -t 'modbus/example-device/status/example_coil/set' -m true
```

Accepted write payload formats:

```json
true
```

```json
{"value": true}
```

## 5. Healthcheck

Use the built-in CLI check to validate config parsing and MQTT TCP reachability:

```bash
modbus-mqtt-bridge --healthcheck --config config/config.yml
```

Containerized version:

```bash
docker compose exec modbus-mqtt-bridge /modbus-mqtt-bridge --healthcheck --config /app/config/config.yml
```

## Next steps

- [Configuration](./configuration) for full field reference
- [Topic Contract](./topic-contract) for MQTT path and payload rules
- [Deployment](./deployment) for production container setups
- [Troubleshooting](./troubleshooting) when values do not flow as expected
