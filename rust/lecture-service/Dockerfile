FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin lecture
WORKDIR /lecture
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/lecture_service*
RUN cargo build

FROM debian:buster-slim

RUN mkdir lecture
WORKDIR /lecture


RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /lecture/target/debug/lecture-service .
COPY --from=build /lecture/.env .
COPY --from=build /lecture/diesel.toml .

EXPOSE 8081

CMD ["/lecture/lecture-service"]