# Configuration

The bridge reads config from `config/config.yml` by default.

Supported file formats:

- `.yml` / `.yaml`
- `.json`

If the configured file path does not exist, the service auto-generates a starter config.

## Full example

```yml
mqtt:
  host: localhost
  port: 1883
  client_id: modbus-mqtt-bridge
  username:
  password:
  base_topic: modbus
  keep_alive_secs: 30
  reconnect_delay_secs: 5

logging:
  level: info
  json: false

sources:
  - id: example-device
    host: 127.0.0.1
    port: 502
    unit_id: 1
    poll_interval_ms: 2000
    request_timeout_ms: 3000
    modbus_retries: 3
    modbus_retry_backoff_ms: 250
    points:
      - name: example_float
        topic: telemetry/example_float
        address: 74
        kind: holding
        data_type: f32
        access: read_write
        count:
        encoding:
          byte_order: big
          word_order: little
        scale:
        offset:
        retain: true
```

## Top-level fields

### `mqtt` (required)

- `host` (required, string): broker hostname or IP
- `port` (optional, integer, default: `1883`)
- `client_id` (optional, string, default: `modbus-mqtt-bridge`)
- `username` (optional, string)
- `password` (optional, string)
- `base_topic` (optional, string, default: `modbus`)
- `keep_alive_secs` (optional, integer, default: `30`)
- `reconnect_delay_secs` (optional, integer, default: `5`)

### `sources` (required, array, at least one entry)

Each source is one Modbus TCP endpoint.

### `logging` (optional object)

- `level` (optional, string, default: `info`)
  - common values: `trace`, `debug`, `info`, `warn`, `error`
- `json` (optional, boolean, default: `false`)

## `sources[]` fields

- `id` (required, string): source identifier used in MQTT topic paths
- `host` (required, string): Modbus endpoint hostname/IP
- `port` (optional, integer, default: `502`)
- `unit_id` (required, integer, `0..255`): Modbus unit/slave ID
- `poll_interval_ms` (optional, integer, default: `1000`)
- `request_timeout_ms` (optional, integer, default: `3000`)
- `modbus_retries` (optional, integer, default: `0`)
  - number of additional retry attempts per Modbus read/write
- `modbus_retry_backoff_ms` (optional, integer, default: `250`)
  - base delay for exponential retry backoff
- `points` (required, array, at least one entry)

## `sources[].points[]` fields

- `name` (required, string): human-readable point name
- `topic` (required, string): topic segment under `<base_topic>/<source_id>/...`
- `address` (required, integer): Modbus start register/coil address
- `kind` (required, enum): `coil`, `discrete_input`, `holding`, `input`
- `data_type` (required, enum): `bool`, `u16`, `i16`, `u32`, `i32`, `f32`, `string`, `raw_u16`
- `access` (optional, enum, default: `read_write`)
  - values: `read_only`, `write_only`, `read_write`
- `count` (optional, integer)
  - overrides automatic register count inference
- `encoding` (optional object)
  - `byte_order` (optional, enum, default: `big`): `big`, `little`
  - `word_order` (optional, enum, default: `big`): `big`, `little`
- `scale` (optional, number)
  - read transformation: `value = value * scale`
- `offset` (optional, number)
  - read transformation after scale: `value = value + offset`
- `retain` (optional, boolean, default: `true`)
  - MQTT retain flag for state publishes

## Automatic register count behavior

If `count` is not set, register count is inferred from `data_type`:

- `bool`, `u16`, `i16`, `string`, `raw_u16` -> `1`
- `u32`, `i32`, `f32` -> `2`

Use `count` explicitly when:

- your device stores string data across multiple registers
- you need a custom width for `raw_u16`

## Topic mapping

For a point with:

- `base_topic = modbus`
- `source.id = boiler`
- `point.topic = telemetry/temp`

topics are:

- state: `modbus/boiler/telemetry/temp`
- write: `modbus/boiler/telemetry/temp/set`

## Write payload format

The `/set` topic accepts either:

```json
42.5
```

or:

```json
{"value": 42.5}
```

## Validation rules

The config loader enforces:

- at least one source
- each source must have at least one point
