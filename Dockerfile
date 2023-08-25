FROM rustlang/rust:nightly-bookworm-slim AS builder

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

RUN ldd --version

WORKDIR /app

COPY . .

RUN --mount=type=cache,id=s/5c1d818e-af74-40ea-8dfb-6d36cd409d9a-/root/cargo/git,target=/root/.cargo/git \
    --mount=type=cache,id=s/5c1d818e-af74-40ea-8dfb-6d36cd409d9a-/root/cargo/registry,target=/root/.cargo/registry \
    cargo build --release --package tyme-discord

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/tyme-discord /tyme-discord

CMD ["/tyme-discord"]
