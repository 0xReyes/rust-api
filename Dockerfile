FROM rust:1.77-slim as builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/rust-api /usr/local/bin/rust-api
CMD ["rust-api"]
