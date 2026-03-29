# Troubleshooting

If data is not flowing as expected, isolate by layer: config -> MQTT -> Modbus -> payload format.

## Quick checklist

Check:

- MQTT broker reachability and credentials
- Modbus host, unit ID, register address, and function type
- exact topic match between config and publisher/subscriber
- point access mode before writing to `/set`
- logs for timeout/decode/payload errors

## Quick isolation commands

Subscribe to all bridge traffic:

```bash
mosquitto_sub -h <mqtt-host> -t 'modbus/#' -v
```

Send a write command:

```bash
mosquitto_pub -h localhost -t 'modbus/example-device/controls/pump_enable/set' -m true
```

Run built-in healthcheck:

```bash
modbus-mqtt-bridge --healthcheck --config config/config.yml
```

## Common failure patterns

### No MQTT messages at all

- wrong broker host/port
- auth mismatch (`username`/`password`)
- bridge not started with expected config file

### Writes ignored on `/set`

- `access` is `read_only` for that point
- published topic does not exactly match `<state-topic>/set`
- payload is not valid JSON

### Values look wrong (byte/word order)

- check `encoding.byte_order` and `encoding.word_order`
- verify `data_type` and optional `count` match device register map
- validate `scale`/`offset` transforms

### Frequent timeout warnings

- raise `request_timeout_ms`
- reduce polling pressure (`poll_interval_ms`, number of points)
- enable retries (`modbus_retries`, `modbus_retry_backoff_ms`)
