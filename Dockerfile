# Use latest rust
FROM rust:latest

# Set timezone
ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Build Dependencies
WORKDIR /telescope
COPY ./Cargo.* ./
RUN mkdir src
RUN echo "fn main() {println!(\"BUILD ARTIFACT! \");}" > src/main.rs
RUN cargo build --release
RUN rm -r target/release/deps/telescope*

# Build telescope proper
COPY ./src ./src
COPY ./graphql ./graphql
RUN cargo build --release
# Build documentation
RUN cargo doc

# Copy over statically served files.
COPY ./static ./static

# Move the telescope executable to the working directory
RUN mv ./target/release/telescope ./telescope
# Move generated docs to statically served folder
RUN mv ./target/doc/ ./static/internal_docs
# Remove all other build artifacts
RUN rm -r ./target

# Copy over templates
COPY ./templates ./templates

# Expose telescope's ports
EXPOSE 80
# Run telescope
ENTRYPOINT ./telescope
