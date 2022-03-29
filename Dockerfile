FROM rust:latest as builder
WORKDIR /app
COPY app/ .
RUN cargo install --path . --root .

FROM debian:buster-slim as runner
COPY --from=builder /app/bin/app /usr/local/bin/rust-rocket-app
CMD rust-rocket-app