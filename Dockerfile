FROM rust:1-slim-buster AS builder

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

WORKDIR /app

COPY . .

RUN --mount=type=cache,id=tKk1ioHC9sU-/root/cargo/git,target=/root/.cargo/git \
    --mount=type=cache,id=tKk1ioHC9sU-/root/cargo/registry,target=/root/.cargo/registry \
    cargo build --release --package tyme-discord

FROM debian:buster-slim

COPY --from=builder /app/target/release/tyme-discord /tyme-discord

CMD ["/tyme-discord"]