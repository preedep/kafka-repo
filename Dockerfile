# Step 1: Build the application
FROM rust:alpine AS builder

RUN apk update && apk add --no-cache \
    openssl \
    openssl-dev \
    musl-dev \
    pkgconfig \
    build-base

# Create app directory
WORKDIR /app

# Copy the source code into the container
COPY Cargo.toml ./
COPY src ./src


# Build the application
RUN cargo build --release

# Step 2: Create the runtime image
FROM alpine:latest

# Install necessary runtime dependencies
RUN apk add --no-cache libgcc libstdc++ openssl ca-certificates

# Create a user to run the application
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Set the working directory
WORKDIR /app

# Copy the built application from the builder
COPY --from=builder /app/target/release/kafka-repo kafka-repo

COPY .env .env
COPY Kafka_Topic_Inventory_D111165.csv Kafka_Topic_Inventory_D111165.csv
COPY Consumer_Group_E-Kafka_list_D200124.csv Consumer_Group_E-Kafka_list_D200124.csv

COPY ./statics ./statics

# Change ownership of the application binary
RUN chown appuser:appgroup kafka-repo

# Switch to the non-root user
USER appuser

EXPOSE 8888
# Run the application
ENTRYPOINT ["/app/kafka-repo"]