FROM rust:1.92-slim-bookworm AS builder
WORKDIR /app

COPY src/ src/
COPY Cargo.lock ./
COPY Cargo.toml ./

RUN cargo build --release

FROM debian:12-slim
WORKDIR /app

COPY --from=builder /app/target/release/rust-api /app/rust-api
COPY config.toml ./

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

CMD ["./rust-api"]
