# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR app

FROM chef AS planner
COPY app/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY app/ .
RUN cargo build --release --bin exobuilds-website

FROM ubuntu:22.04 as runner
RUN apt-get -qy update && apt-get upgrade
COPY --from=builder /app/target/release/exobuilds-website /usr/local/bin/exobuilds-website
COPY --from=builder /app/public /public
COPY --from=builder /app/templates /templates
CMD exobuilds-website
