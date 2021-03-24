# Use Rust 1.50 (we use library functions that were stablized in 1.50 and cannot use any previous version)
FROM rust:1.50-buster

# Set timezone
ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Build Dependencies
WORKDIR /telescope
COPY ./Cargo.* ./
RUN mkdir src
RUN echo "fn main() {println!(\"BUILD ARTIFACT! \");}" > src/main.rs
RUN cargo build --release
RUN rm -rfv target/release/deps/telescope*

# Build telescope proper
COPY ./src ./src
COPY ./graphql ./graphql
RUN cargo build --release

# Delete unneeded build artifacts;
# Move the telescope executable to the working directory and delete
# everything else.
RUN mv ./target/release/telescope ./telescope
RUN rm -r ./target

# Copy over remaining files needed to run.
COPY ./static ./static
COPY ./templates ./templates

# Expose telescope's ports
EXPOSE 80
# Run telescope
ENTRYPOINT ./telescope
