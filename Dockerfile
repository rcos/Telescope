# Build stage 1
FROM rust:1-buster AS build
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./Cargo.* /build/
WORKDIR /build/
RUN mkdir .cargo
RUN echo "fn main() {}" > /build/src/main.rs
RUN cargo vendor > .cargo/config

COPY ./src/ /build/src
RUN cargo build --release

COPY ./diesel.toml /build/
COPY ./.env /build/
COPY ./migrations/ /build/migrations
COPY ./templates/ /build/templates
COPY ./static/ /build/static
COPY ./config.toml /build/config.toml
COPY ./tls-ssl/ /build/tls-ssl
COPY ./docker-run.sh /build/docker-run.sh

EXPOSE 8080
EXPOSE 8443

CMD ["/bin/sh", "/build/docker-run.sh"]
