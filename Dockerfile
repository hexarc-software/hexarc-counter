# Build app
FROM rust:slim-bullseye AS builder

RUN USER=root cargo new --bin hexarc-badger
WORKDIR /hexarc-badger

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Rust coverts "-" to "_"
RUN rm ./target/release/deps/hexarc_badger*
RUN cargo build --release

# Pack app
FROM debian:bullseye-slim
WORKDIR /

COPY --from=builder ./hexarc-badger/target/release/hexarc-badger .
CMD ["./hexarc-badger"]