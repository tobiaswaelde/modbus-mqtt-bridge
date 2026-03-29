# Operations

## Health checks

```bash
modbus-mqtt-bridge --healthcheck --config config/config.yml
```

Container command:

```bash
/modbus-mqtt-bridge --healthcheck --config /app/config/config.yml
```

The healthcheck validates:

- config parsing/validation
- TCP reachability to the configured MQTT host and port

## Logging

Enable JSON logs in config:

- `logging.json: true`

- Override log level with environment:

```bash
RUST_LOG=debug modbus-mqtt-bridge --config config/config.yml
```

Useful values: `trace`, `debug`, `info`, `warn`, `error`.

## Safe config rollouts

1. Validate edited config locally with `--healthcheck`.
2. Restart bridge.
3. Verify state publish topics and `/set` write topics.
4. Watch logs for `timeout`, `decode`, or `payload` errors.

## Runtime behavior notes

- Each source polls independently, so one slow endpoint should not block others.
- Read failures are logged and retried in subsequent poll cycles.
- MQTT reconnect behavior runs continuously in the event loop.
