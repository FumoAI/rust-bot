FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
FROM ubuntu:22.04
WORKDIR /app
COPY --from=builder /app/target/release/rust-bot /app/rust-bot
COPY kovi.conf.toml /app/kovi.conf.toml
CMD ["/app/rust-bot"]
