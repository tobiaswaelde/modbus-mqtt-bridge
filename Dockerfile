FROM rust:1.87-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd --system --create-home --home-dir /app appuser
WORKDIR /app

COPY --from=builder /app/target/release/modbus-mqtt-bridge /usr/local/bin/modbus-mqtt-bridge
COPY config /app/config

USER appuser
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD ["/usr/local/bin/modbus-mqtt-bridge", "--healthcheck", "--config", "/app/config/config.yml"]
ENTRYPOINT ["modbus-mqtt-bridge"]
CMD ["--config", "/app/config/config.yml"]
