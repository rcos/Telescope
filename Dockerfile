FROM rust:1-buster
COPY . /build/
WORKDIR /build/
RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup
EXPOSE 8080
EXPOSE 8443

ENTRYPOINT [ "/build/target/release/telescope" ]
