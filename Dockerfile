# Build stage 1
FROM rust:1-buster AS build
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./migrations/ /build/migrations
COPY ./Cargo.* /build/
COPY ./diesel.toml /build/
COPY ./.env /build/
COPY ./src/ /build/src
WORKDIR /build/
RUN diesel setup
RUN diesel migrations run
RUN cargo build --release

COPY ./templates/ /build/templates
COPY ./static/ /build/static
EXPOSE 8080
EXPOSE 8443
ENTRYPOINT [ "/build/target/release/telescope" ]
