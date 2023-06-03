FROM rust:1-slim-buster AS builder

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/tyme-discord /tyme-discord

CMD ["/tyme-discord"]
