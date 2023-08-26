FROM rustlang/rust:nightly-bookworm-slim AS builder

ARG DATABASE_URL

WORKDIR /app

COPY . .

RUN cargo build --release --package tyme-discord

FROM debian:bookworm-slim AS runtime

COPY --from=builder /app/target/release/tyme-discord /tyme-discord

CMD ["/tyme-discord"]
