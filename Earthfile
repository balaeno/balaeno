VERSION 0.8
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust
 
source:
    FROM rust:slim-bookworm
    WORKDIR /workdir
    DO rust+INIT --keep_fingerprints=true
    COPY --keep-ts Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir cli libruntime ./

build:
    FROM +source
    WORKDIR /workdir/balaeno
    DO rust+CARGO --args="build --release" --output="release/[^/\.]+"
    SAVE ARTIFACT ./target/release/* AS LOCAL ./target/release/