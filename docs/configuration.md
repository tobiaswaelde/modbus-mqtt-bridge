# Configuration

The service supports YAML or JSON config. Default path is `config/config.yml`.
If the file is missing, a safe template is generated automatically.

```yml
mqtt:
  host: localhost
  port: 1883
  client_id: modbus-mqtt-bridge
  base_topic: modbus
  reconnect_delay_secs: 5

logging:
  level: info
  json: false

metrics:
  enabled: false
  bind: 0.0.0.0:9464

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
        address: 0
        kind: holding
        data_type: f32
        access: read_write
```

## Supported enums

- `kind`: `coil`, `discrete_input`, `holding`, `input`
- `data_type`: `bool`, `u16`, `i16`, `u32`, `i32`, `f32`, `string`, `raw_u16`
- `access`: `read_only`, `write_only`, `read_write`
- `encoding.byte_order` / `encoding.word_order`: `big`, `little`
