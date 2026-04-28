# --- Build stage ---
FROM rust:1.91-bookworm AS builder

WORKDIR /src

COPY . .
RUN cargo build --release


# --- Runtime stage ---
FROM debian:trixie-slim

# Default config path inside the container.
# You can bind-mount a single file from the host, e.g.:
#   -v ./config.toml:/config/config.toml
ENV CONFIG_PATH=/config/config.toml

COPY --from=builder /src/target/release/ECUEmulator /usr/local/bin/ecuemulator
COPY data/sample_config.toml /config/sample_config.toml
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
# Default: run emulator with the config file
CMD []
