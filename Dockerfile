# Build stage 1
FROM rust:1-buster AS build
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./migrations /build/
COPY ./diesel.toml /build/
COPY ./.env /build/
COPY ./src /build/
WORKDIR /build/
RUN diesel setup
RUN diesel migrations run

COPY . .
RUN cargo build --release

EXPOSE 8080
EXPOSE 8443
ENTRYPOINT [ "/build/target/release/telescope" ]
