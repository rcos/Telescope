# Build stage 1
FROM rust:1-buster AS build
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./migrations/ /build/migrations
COPY ./Cargo.* /build/
COPY ./diesel.toml /build/
COPY ./.env /build/
COPY ./src/ /build/src
WORKDIR /build/

RUN cargo build --release

EXPOSE 8080
EXPOSE 8443

COPY ./templates/ /build/templates
COPY ./static/ /build/static
COPY ./docker-run.sh /build/docker-run.sh

ENTRYPOINT ["/bin/sh", "/build/docker_run.sh"]
