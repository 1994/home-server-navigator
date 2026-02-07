# Home Server Navigator Dockerfile
# Multi-stage build for minimal image size

# Stage 1: Build frontend
FROM node:20-alpine AS frontend-builder

WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# Stage 2: Build backend
FROM rust:1.75-alpine AS backend-builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app/backend
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
COPY backend/build.rs ./

# Copy frontend build for embedding
COPY --from=frontend-builder /app/frontend/dist ./frontend-dist

# Build static binary
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 3: Final minimal image
FROM scratch

# Copy CA certificates for HTTPS
COPY --from=backend-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy binary
COPY --from=backend-builder /app/backend/target/x86_64-unknown-linux-musl/release/home-server-navigator /home-server-navigator

# Data directory (will be mounted as volume)
VOLUME ["/data"]

# Expose port
EXPOSE 8080

# Run as non-root (scratch doesn't support user)
# Use numeric uid for compatibility
USER 1000:1000

ENTRYPOINT ["/home-server-navigator"]
CMD ["--host", "0.0.0.0", "--port", "8080", "--data-file", "/data/services.json"]
