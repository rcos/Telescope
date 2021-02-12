# Use Rust 1.50 (we use library functions that were stablized in 1.50 and cannot use any previous version)
FROM rust:1.50-buster
# Build Dependencies
WORKDIR /telescope/build
COPY ./Cargo.* ./
RUN mkdir src
RUN echo "fn main() {println!(\"BUILD ARTIFACT! \");}" > src/main.rs
RUN cargo build --release
RUN rm -rfv target/release/deps/telescope*
# Build telescope proper
COPY ./src ./src
RUN cargo build --release
# Copy over remaining files needed to run.
COPY ./static ./static
COPY ./templates ./templates
COPY ./tls-ssl ./tls-ssl
COPY ./.env .
COPY ./config.toml .
# Expose telescope's ports
EXPOSE 80
EXPOSE 443
# Run telescope
CMD ["cargo", "run", "--release"]
