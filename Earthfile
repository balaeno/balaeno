VERSION 0.8
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust
 
source:
    FROM rust:slim-bookworm
    WORKDIR /workdir
    RUN rustup component add clippy
    DO rust+INIT --keep_fingerprints=true
    COPY --keep-ts --if-exists Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir crates crates

build:
    FROM +source
    WORKDIR /workdir
    DO rust+CARGO --args="build --release" --output="release/[^/\.]+"
    SAVE ARTIFACT ./target/release/* AS LOCAL ./target/release/

clippy:
    FROM +source
    WORKDIR /workdir
    DO rust+CARGO --args="clippy --all-targets --all-features -- -D warnings"
