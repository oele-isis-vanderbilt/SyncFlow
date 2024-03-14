FROM rust:slim as builder

WORKDIR ./app

COPY . .

RUN apt-get update && apt-get install libpq-dev pkg-config -y && cargo build --bin api --release --package api

FROM ubuntu:latest

ARG APP=/app

EXPOSE 8081

ENV TZ=Etc/UTC \
    APP_USER=livekit

RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER

RUN apt-get update && apt-get install tzdata libpq-dev -y && rm -rf /var/cache/apk/*

WORKDIR $APP

COPY --from=builder /app/target/release/api .

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER

ENTRYPOINT ["./api"]

CMD ["./api"]
