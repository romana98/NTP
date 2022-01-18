FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin staff
WORKDIR /staff
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/staff_service*
RUN cargo build

FROM debian:buster-slim

RUN mkdir staff
WORKDIR /staff


RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /staff/target/debug/staff-service .
COPY --from=build /staff/.env .
COPY --from=build /staff/diesel.toml .

EXPOSE 8082

CMD ["/staff/staff-service"]