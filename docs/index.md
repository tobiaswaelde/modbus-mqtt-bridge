# Modbus MQTT Bridge

A lightweight Rust service that bridges Modbus TCP and MQTT.

## What it does

- Polls configured Modbus points
- Publishes raw JSON values to MQTT state topics
- Accepts writes through matching `/set` topics
- Supports Docker deployment with built-in health checks

## Read next

- [Getting Started](./getting-started)
- [Concepts](./concepts)
- [Architecture](./architecture)
- [Recipes](./recipes)
- [Configuration](./configuration)
- [Deployment](./deployment)
