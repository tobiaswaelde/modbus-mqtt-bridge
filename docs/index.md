---
layout: home
hero:
  name: Modbus MQTT Bridge
  text: Reliable Modbus TCP ↔ MQTT integration
  tagline: Poll Modbus points, publish JSON state topics, and handle writes via matching /set topics.
  image:
    src: /logo.svg
    alt: Modbus MQTT Bridge logo
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started
    - theme: alt
      text: Configuration Reference
      link: /configuration
    - theme: alt
      text: Deployment Guide
      link: /deployment
features:
  - title: Deterministic Topic Contract
    details: "Every writable point has a predictable pair of topics: state publish and matching /set write endpoint."
  - title: Built for Operations
    details: Includes retry/backoff controls, structured logging options, and healthcheck mode for runtime monitoring.
  - title: Production-Ready Containers
    details: Multi-stage Docker build with a small scratch runtime image and built-in container health checks.
  - title: Fast Integration
    details: Use with Home Assistant, Node-RED, or custom consumers without writing bespoke protocol glue code.
---

## Quick links

- [Getting Started](/getting-started)
- [Concepts](/concepts)
- [Architecture](/architecture)
- [Topic Contract](/topic-contract)
- [Troubleshooting](/troubleshooting)
- [Operations](/operations)
