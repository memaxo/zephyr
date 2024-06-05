FROM rust:1.56 as builder
WORKDIR /usr/src/zephyrchain
# Copy the source tree and the Cargo.lock and Cargo.toml to use them for caching
COPY ./Cargo.lock ./Cargo.toml ./
# Create a dummy project and build the project's dependencies
# The project is named zephyrchain, so the binary will be found at target/release/zephyrchain
RUN mkdir src/ \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release \
    && rm -f target/release/deps/zephyrchain*
# Now that the dependencies are built, copy your actual source code
COPY ./src ./src
# Build your application
RUN cargo build --release

# The final base image
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
# Copy the binary from the builder stage
COPY --from=builder /usr/src/zephyrchain/target/release/zephyrchain /usr/local/bin/zephyrchain
# Set the startup command to run your binary
CMD ["zephyrchain"]
