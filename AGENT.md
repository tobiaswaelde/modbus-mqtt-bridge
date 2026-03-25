# AGENT.md

This file gives development guidance for AI coding agents (Codex, Claude, etc.) working on this repository.

## Project Summary

- Language: Rust
- Service: Modbus TCP <-> MQTT bridge
- Runtime: single binary, Docker-friendly
- Default config path: `config/config.yml`

Core behavior:

- Poll configured Modbus points and publish plain JSON values to MQTT state topics.
- Subscribe to MQTT `/set` topics for writable points and write incoming values back to Modbus.

## Code Layout

- `src/main.rs`
  - CLI entrypoint
  - config bootstrap (creates starter config if missing)
  - logging setup
  - healthcheck mode (`--healthcheck`)
- `src/config.rs`
  - strongly typed config model
  - defaults and validation
- `src/bridge.rs`
  - MQTT event loop
  - polling loops
  - topic handling
  - Modbus read/write orchestration
- `src/modbus_codec.rs`
  - Modbus register encode/decode logic
  - byte/word order handling
  - payload coercion
- `config/config.yml`
  - safe template config for local/dev use

## Development Rules

1. Keep the service resilient.
   - Prefer graceful degradation over whole-loop failure.
   - Avoid panics in runtime paths.
   - When adding I/O operations, include timeout and retry behavior where appropriate.

2. Preserve topic contract unless explicitly changed.
   - State: `<base_topic>/<source_id>/<point_topic>`
   - Writes: `<base_topic>/<source_id>/<point_topic>/set`
   - State payload should remain the raw value JSON, not wrapped metadata.

3. Keep config backward-compatible when possible.
   - Add new fields as optional with defaults.
   - Update `config::AppConfig::validate` for new invariants.
   - Reflect new options in both `config/config.yml` and `README.md`.

4. Treat secrets safely.
   - Never commit real broker credentials, tokens, or production IPs.
   - Keep checked-in config as placeholders/example values.

5. Keep comments high-signal.
   - Explain why, not obvious what.
   - Avoid noisy or redundant comments.

## Workflow Expectations

When making changes:

1. Update code.
2. Run:
   - `cargo fmt --all`
   - `cargo clippy --all-targets --all-features -- -D warnings` (if available)
   - `cargo test`
3. Update docs/config examples if behavior or options changed.

For larger changes, prefer multiple logical commits with meaningful messages.

## Docker and CI Notes

- `Dockerfile` includes a healthcheck that runs binary `--healthcheck`.
- GitHub Actions:
  - `.github/workflows/test.yml` for formatting/lint/tests/build
  - `.github/workflows/docker.yml` for GHCR image publish
- Docker image naming in CI is derived from GitHub repository owner/name.

## Good First Improvements

- Add integration tests with a mock Modbus server and test MQTT broker.
- Add optional MQTT TLS settings in config.
- Add optional publish QoS setting per point or globally.
- Add metrics endpoint (for example Prometheus) for poll success/failure counts and latencies.

