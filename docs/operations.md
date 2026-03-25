# Operations

- Manual health check:

```bash
modbus-mqtt-bridge --healthcheck --config config/config.yml
```

- Enable metrics endpoint:
  - `metrics.enabled: true`
  - default bind: `0.0.0.0:9464`

- Enable JSON logs:
  - `logging.json: true`

- Override log level with environment:

```bash
RUST_LOG=debug modbus-mqtt-bridge --config config/config.yml
```
