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
COPY ./graphql ./graphql
RUN cargo build --release
# Copy over remaining files needed to run.
COPY ./static ./static
COPY ./templates ./templates
COPY ./tls-ssl ./tls-ssl
# Do not copy .env file to the Telescope docker container.
# Any environment variables that need to be set should be put in the config.toml
# if possible, or passed to the telescope container using the docker-compose
# file.
#COPY ./.env .
COPY ./config.toml .
# Expose telescope's ports
EXPOSE 80
EXPOSE 443
# Run telescope
CMD ["cargo", "run", "--release"]
