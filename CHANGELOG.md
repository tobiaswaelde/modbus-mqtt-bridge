# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

## [1.0.2] - 2026-04-25

### Added

### Changed

### Fixed

- Prevented the MQTT event loop from stalling after reconnect when subscription requests are queued under load.
- Made `/set` command forwarding non-blocking in the MQTT event loop to avoid head-of-line blocking.

## [1.0.1] - 2026-03-30

### Added

- Release workflow now builds and pushes Docker images to GitHub Container Registry (`ghcr.io`) on tag releases.

### Changed

- Strengthened runtime config validation with checks for empty/invalid MQTT and source fields.
- Added validation for duplicate source IDs and duplicate point topics per source.
- Added validation for invalid MQTT topic patterns (wildcards and `/set` suffix in point topics).

### Fixed

- Prevented invalid zero-valued polling/timeout config values from being accepted.

## [1.0.0] - 2026-03-25

### Added

- VitePress-based documentation site with Markdown pages and Mermaid architecture diagrams.
- Dedicated deployment docs including `docker compose` and `docker run` examples.
- Docs lockfile (`docs/package-lock.json`) for reproducible dependency installs.
- Docs build job in CI (`test.yml`) to validate documentation on push and pull requests.

### Changed

- Simplified runtime by removing optional metrics endpoint code and configuration.
- Updated GitHub Pages workflow to build VitePress output from `docs/`.
- Updated release workflow to build and package a musl binary (`x86_64-unknown-linux-musl`).
- Streamlined README to keep it concise and link to docs for detailed guidance.

### Fixed

- Improved CI determinism by switching docs install steps to `npm ci` with lockfile-aware cache paths.

## [0.2.0] - 2026-03-25

### Added

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
