# Build stage: compile a fully static binary to enable a tiny runtime image.
FROM rust:1.87-alpine AS builder
WORKDIR /app

# Install minimal native tooling needed for musl static builds.
RUN apk add --no-cache musl-dev

# Build for musl so the runtime can be `scratch` (no libc required at runtime).
RUN rustup target add x86_64-unknown-linux-musl

# Copy manifest files first to maximize layer cache reuse.
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Copy application sources and bundled default config used by include_str! at compile time.
COPY src src
COPY config config
RUN cargo build --release --target x86_64-unknown-linux-musl \
  && strip /app/target/x86_64-unknown-linux-musl/release/modbus-mqtt-bridge

# Runtime stage: `scratch` is the smallest possible base image.
FROM scratch
WORKDIR /app

# Copy in the release binary and default config template.
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/modbus-mqtt-bridge /modbus-mqtt-bridge
COPY config /app/config

# Run as an unprivileged numeric UID/GID (no passwd files needed in scratch).
USER 65532:65532

# Healthcheck calls the built-in CLI check, which validates config and MQTT broker reachability.
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD ["/modbus-mqtt-bridge", "--healthcheck", "--config", "/app/config/config.yml"]

ENTRYPOINT ["/modbus-mqtt-bridge"]
# Default config path can be overridden by passing a different --config value.
CMD ["--config", "/app/config/config.yml"]
