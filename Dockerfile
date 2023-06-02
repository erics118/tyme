FROM rust:1-slim-buster AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --package tyme-discord

FROM debian:buster-slim

COPY --from=builder /app/target/release/tyme-discord /tyme-discord
CMD ["/tyme-discord"]