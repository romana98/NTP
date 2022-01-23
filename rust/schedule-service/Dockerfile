FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin schedule
WORKDIR /schedule
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/schedule_service*
RUN cargo build

FROM debian:buster-slim

RUN mkdir schedule
WORKDIR /schedule


RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /schedule/target/debug/schedule-service .
COPY --from=build /schedule/.env .
COPY --from=build /schedule/diesel.toml .

EXPOSE 8084

CMD ["/schedule/schedule-service"]