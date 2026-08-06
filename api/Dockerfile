# syntax=docker/dockerfile:1
#
# Diesel's "postgres" feature links against libpq (unlike the sqlx/rustls
# services elsewhere in this repo), so both stages need libpq installed.

FROM rust:1-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --bin api \
    && cp /app/target/release/api /app/api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/api /app/api

EXPOSE 8080
USER nobody
ENTRYPOINT ["/app/api"]
