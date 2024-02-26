# build stage
FROM rust:slim as builder

RUN apt-get update  \
    && apt-get install -y musl-tools  \
    && rm -rf /var/lib/apt/lists/* && \
    USER=root cargo new --bin app

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build && rm src/*.rs

COPY ./src ./src

RUN rm -rf ./target/* && cargo build --release

FROM ubuntu:latest

RUN mkdir /app

WORKDIR /app

RUN apt-get update

COPY --from=builder /app/target/release/livekit-mmla-api .

EXPOSE 8081

CMD ["/app/livekit-mmla-api"]