
export RUST_BACKTRACE := '1'

export JUST_LOG := "warn"

clean:
    cargo clean

alias t := test
test:
    cargo test -- --nocapture

alias r := run
run:
    cargo run

alias b := build
build:
    cargo build

alias f := fmt
fmt:
    cargo fmt --all

clippy:
    cargo clippy --all --all-targets --all-features

alias c := check
check: fmt clippy test
