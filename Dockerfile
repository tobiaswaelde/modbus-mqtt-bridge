# Build stage: compile the Rust binary with all source/config assets available.
FROM rust:1.87-bookworm AS builder
WORKDIR /app

# Copy manifest files first to maximize layer cache reuse for dependencies.
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Copy application sources and bundled default config used by include_str! at compile time.
COPY src src
COPY config config
RUN cargo build --release

# Runtime stage: small base image that only contains the compiled binary and config directory.
FROM debian:bookworm-slim
RUN useradd --system --create-home --home-dir /app appuser
WORKDIR /app

# Copy in the release binary and default config template.
COPY --from=builder /app/target/release/modbus-mqtt-bridge /usr/local/bin/modbus-mqtt-bridge
COPY config /app/config

USER appuser
# Healthcheck calls the built-in CLI check, which validates config and MQTT broker reachability.
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD ["/usr/local/bin/modbus-mqtt-bridge", "--healthcheck", "--config", "/app/config/config.yml"]
ENTRYPOINT ["modbus-mqtt-bridge"]
# Default config path can be overridden by passing a different --config value.
CMD ["--config", "/app/config/config.yml"]
