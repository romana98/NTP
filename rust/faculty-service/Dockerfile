FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin faculty
WORKDIR /faculty
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/faculty_service*
RUN cargo build

FROM debian:buster-slim

RUN mkdir faculty
WORKDIR /faculty


RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /faculty/target/debug/faculty-service .
COPY --from=build /faculty/.env .
COPY --from=build /faculty/diesel.toml .

EXPOSE 8083

CMD ["/faculty/faculty-service"]