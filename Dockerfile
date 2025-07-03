# Rustodon Dockerfile - Optimized Build Version
# Author: arkSong (arksong2018@gmail.com)
# Project: rustodon
#
# Build: docker build -t rustodon .
# Run: docker run --rm -e DATABASE_URL=postgres://rustodon:rustodon@host.docker.internal:5432/rustodon -p 3000:3000 rustodon
#
# This Dockerfile uses a multi-stage build approach for optimal image size and security

# ---- Build Stage ----
FROM rust:1.75-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p rustodon-server/src && \
    echo "fn main() {}" > rustodon-server/src/main.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release -p rustodon-server

# Copy actual source code
COPY . .

# Touch main.rs to ensure it's newer than the dummy
RUN touch rustodon-server/src/main.rs

# Build the actual application
RUN cargo build --release -p rustodon-server

# ---- Runtime Stage ----
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd -r rustodon && useradd -r -g rustodon rustodon

# Copy binary from builder stage
COPY --from=builder /app/target/release/rustodon-server /usr/local/bin/rustodon-server

# Set ownership
RUN chown rustodon:rustodon /usr/local/bin/rustodon-server

# Switch to non-root user
USER rustodon

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Default command
CMD ["rustodon-server"]
