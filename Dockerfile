FROM rust:latest as builder
WORKDIR /app
COPY app/ .
RUN cargo install --path . --root .

FROM debian:buster-slim as runner
COPY --from=builder /app/bin/exobuilds-website /usr/local/bin/exobuilds-website
COPY --from=builder /app/public /public
COPY --from=builder /app/templates /templates
CMD exobuilds-website
