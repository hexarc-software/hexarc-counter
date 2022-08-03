# Build app
FROM rust:slim-bullseye AS builder

RUN USER=root cargo new --bin hexarc-tracker
WORKDIR /hexarc-tracker

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build and cache dependencies
RUN cargo build --release

# Cleanup temp files
RUN rm src/*.rs
RUN rm ./target/release/deps/hexarc_tracker*

# Build app
COPY ./src ./src
RUN cargo build --release

# Pack app
FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder ./hexarc-tracker/target/release/hexarc-tracker .
CMD PORT=$PORT ./hexarc-tracker