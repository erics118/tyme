alias t := test

alias c := check

export RUST_BACKTRACE := '0'

export JUST_LOG := "warn"

test:
    cargo test

run:
    cargo run

build:
    cargo build

fmt:
    cargo fmt --all

check: fmt clippy test

clippy:
    cargo clippy --all --all-targets --all-features
