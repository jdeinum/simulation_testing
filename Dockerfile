FROM rust:latest as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release --bin simulation_testing_bin

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/simulation_testing_bin /app/

# Copy configuration directory
COPY configuration ./configuration

# Expose port (will be overridden per container)
EXPOSE 8000

# The specific config will be passed as an argument
CMD ["./simulation_testing_bin"]