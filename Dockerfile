FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin rust_microservice_starter_kit

# We do not need the Rust toolchain to run the binary!
FROM ubuntu:22.04 AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y openssl libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust_microservice_starter_kit /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rust_microservice_starter_kit"]
EXPOSE 3000
