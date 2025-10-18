# -------- Build Stage --------
FROM rust:1.82 AS builder

# Install required tools for MUSL builds
RUN apt-get update && apt-get install -y musl-tools pkg-config build-essential ca-certificates

# Set work directory
WORKDIR /usr/src/my_api

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Copy actual source and build release binary
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Runtime Stage --------
FROM alpine:latest

# Add CA certificates (needed for HTTPS with reqwest)
RUN apk add --no-cache ca-certificates

WORKDIR /usr/local/bin

# Copy binary from builder stage
COPY --from=builder /usr/src/my_api/target/x86_64-unknown-linux-musl/release/stage0-profile .

# Expose your app’s port
EXPOSE 3000

# Run the binary
CMD ["./stage0-profile"]
