# Getting Started

1. Clone the repository.
2. Edit `config/config.yml` with your Modbus and MQTT values.
3. Start your MQTT broker.
4. Run:

```bash
cargo run -- --config config/config.yml
```

5. Subscribe to state topics and publish a test payload to a `/set` topic.
