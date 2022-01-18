FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin auth
WORKDIR /auth
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
COPY ./diesel.toml .
RUN touch .env; 

RUN rm ./target/debug/deps/auth_service*
RUN cargo build

FROM debian:buster-slim

RUN mkdir auth
WORKDIR /auth


RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /auth/target/debug/auth-service .
COPY --from=build /auth/.env .
COPY --from=build /auth/diesel.toml .

EXPOSE 8085

CMD ["/auth/auth-service"]