FROM rust:alpine
COPY . /build/
WORKDIR /build/
RUN apk --no-cache add ca-certificates libpq openssl-dev gcc
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup
RUN diesel migration run
RUN cargo build --release
EXPOSE 8080
EXPOSE 8443

ENTRYPOINT [ "/build/target/release/telescope" ]
