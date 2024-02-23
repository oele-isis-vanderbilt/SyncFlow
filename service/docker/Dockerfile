FROM rust:slim as builder

RUN apt-get update && USER=root cargo new --bin app

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build && rm src/*.rs

COPY ./src ./src

RUN cargo build --release



FROM debian:buster-slim

RUN mkdir app

WORKDIR /app

RUN apt-get update

COPY --from=builder /app/target/release/livekit-mmla-api .

# expose port
EXPOSE 8081


CMD ["/app/livekit-mmla-api"]