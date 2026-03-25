# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

## [0.2.0] - 2026-03-25

### Added

- Optional metrics endpoint (`metrics.enabled`, `metrics.bind`) with Prometheus-style counters.
- Integration test suite in `tests/integration_config_codec.rs`.
- Release workflow (`.github/workflows/release.yml`) for tagged GitHub releases with packaged artifacts.
- Expanded docs website sections: architecture diagram, practical recipes, troubleshooting.

### Changed

- MQTT `/set` topic subscriptions are now refreshed automatically on reconnect (`ConnAck`).
- Modbus operations now support configurable retries and exponential backoff per source.
- Polling loop now logs point-level failures and continues remaining points in the same cycle.
- License file now contains full GPLv3 text.

### Fixed

## [0.1.0] - 2026-03-25

### Added

- Initial Rust implementation of the Modbus TCP <-> MQTT bridge.
- YAML/JSON configuration loading and validation.
- Modbus point mapping for coils, discrete inputs, holding registers, and input registers.
- Data type support: `bool`, `u16`, `i16`, `u32`, `i32`, `f32`, `string`, `raw_u16`.
- MQTT state publishing and `/set` write handling.
- Auto-generated starter config when config file is missing.
- Built-in healthcheck mode for container liveness/readiness checks.
- Docker and Docker Compose setup.
- GitHub Actions workflows for test, Docker publish, and GitHub Pages deployment.
- Documentation website in `docs/`.
- `AGENT.md` contributor guidance for AI-assisted development.

### Changed

- Standardized default config path to `config/config.yml`.
- Improved code comments and added Rustdoc comments for public APIs.
- Updated README with badges, better onboarding, and docs link.
