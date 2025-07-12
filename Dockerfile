# Multi-stage Docker build for Rust application
# Stage 1: Build environment
FROM rust:1.75 as builder

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false homelabme

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/homelabme /app/homelabme

# Change ownership
RUN chown homelabme:homelabme /app/homelabme

# Switch to app user
USER homelabme

# Expose port 80 for production
EXPOSE 80

# Set production environment
ENV PORT=80
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

# Run the application
CMD ["./homelabme"]