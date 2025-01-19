FROM rust:1.84 AS builder

# Set the working directory
WORKDIR /usr/src/tree-ftp

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Use a base image with the required glibc version
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && apt-get install -y \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/tree-ftp/target/release/tree-ftp /usr/local/bin/tree-ftp

# Set the default command
ENTRYPOINT ["tree-ftp"]