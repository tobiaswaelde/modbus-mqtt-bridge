# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

