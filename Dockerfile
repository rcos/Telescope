# Build stage 1
FROM rust:1-buster AS build
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./Cargo.* /build/telescope/
WORKDIR /build/telescope/
RUN mkdir src
RUN echo "fn main() {println!(\"BUILD ARTIFACT\")}" > src/main.rs
RUN cargo build --release
RUN rm -rfv target/release/deps/telescope*

COPY ./src ./src
RUN cargo build --release

COPY ./diesel.toml .
COPY ./.env .
COPY ./migrations/ /build/telescope/migrations
COPY ./templates/ /build/telescope/templates
COPY ./static/ /build/telescope/static
COPY ./config.toml .
COPY ./tls-ssl/ /build/telescope/tls-ssl
COPY ./docker-run.sh .

EXPOSE 8080
EXPOSE 8443

CMD ["/bin/sh", "/build/docker-run.sh"]
