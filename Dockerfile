# Step 1: Build the application
FROM rust:alpine AS builder

# Install dependencies necessary for building the application
RUN apk update && apk add --no-cache \
    openssl \
    openssl-dev \
    musl-dev \
    pkgconfig \
    build-base \
    clang \
    llvm-dev \
    cmake

# Install the x86_64-unknown-linux-musl target
RUN rustup target add x86_64-unknown-linux-musl

# Create app directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to build dependencies first (caching)
COPY Cargo.toml Cargo.lock ./

# Copy the source code into the container
COPY src ./src

# Build the application in release mode targeting x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl -j 1

# Step 2: Create the runtime image
FROM alpine:latest

# Install necessary runtime dependencies
RUN apk add --no-cache libgcc libstdc++ openssl ca-certificates

# Create a user to run the application
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Set the working directory
WORKDIR /app

# Copy the built application from the builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/kafka-repo kafka-repo

COPY .env .env
COPY ./statics ./statics

# Change ownership of the application binary
RUN chown appuser:appgroup kafka-repo

# Switch to the non-root user
USER appuser

EXPOSE 8888
# Run the application
ENTRYPOINT ["/app/kafka-repo"]