FROM rust:slim as build

RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin api
WORKDIR /api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build; \
    rm src/*.rs

COPY ./src ./src
RUN touch .env;

RUN rm ./target/debug/deps/api_gateway*
RUN cargo build

FROM debian:buster-slim

RUN mkdir api
WORKDIR /api

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /api/target/debug/api-gateway .
COPY --from=build /api/.env .

# expose port
EXPOSE 8080

# run the binary
CMD ["/api/api-gateway"]