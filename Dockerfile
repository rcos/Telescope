FROM rust:1-buster
COPY . /build/
WORKDIR /build/
RUN cargo build --release
EXPOSE 8080
EXPOSE 8443

ENTRYPOINT [ "/build/target/release/telescope" ]
