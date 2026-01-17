FROM rust:1.92-slim AS builder
WORKDIR /app

COPY src/ src/
COPY Cargo.lock ./
COPY Cargo.toml ./

RUN cargo build --release

FROM debian:12-slim
WORKDIR /app

COPY --from=builder /app/target/release/rust-api /app/rust-api

CMD ["./rust-api"]
