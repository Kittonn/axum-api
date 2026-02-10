FROM rust:1.93.0-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y cmake build-essential clang libcurl4-openssl-dev protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/axum-api/Cargo.toml ./crates/axum-api/Cargo.toml
COPY crates/user-consumer/Cargo.toml ./crates/user-consumer/Cargo.toml

RUN mkdir -p crates/axum-api/src && echo "fn main() {}" > crates/axum-api/src/main.rs
RUN mkdir -p crates/user-consumer/src && echo "fn main() {}" > crates/user-consumer/src/main.rs

RUN cargo build --release -p user-consumer

RUN rm -rf crates/axum-api/src crates/user-consumer/src
COPY crates ./crates
COPY proto ./proto 

RUN touch crates/user-consumer/src/main.rs 
RUN cargo build --release -p user-consumer

FROM debian:testing-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/user-consumer /usr/local/bin/user-consumer

CMD ["user-consumer"]