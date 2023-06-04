FROM rust:1.70.0-slim-buster AS builder

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

WORKDIR /app

COPY . .

RUN --mount=type=cache,id=s/5c1d818e-af74-40ea-8dfb-6d36cd409d9a-/root/cargo/git,target=/root/.cargo/git \
    --mount=type=cache,id=s/5c1d818e-af74-40ea-8dfb-6d36cd409d9a-/root/cargo/registry,target=/root/.cargo/registry \
    cargo build --release --package tyme-discord

FROM debian:buster-slim

COPY --from=builder /app/target/release/tyme-discord /tyme-discord

CMD ["/tyme-discord"]
