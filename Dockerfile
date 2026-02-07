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
FROM rust:1.78-alpine AS backend-builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig perl make

WORKDIR /app

# Copy backend files
COPY backend/Cargo.toml backend/Cargo.lock ./backend/
COPY backend/src ./backend/src
COPY backend/build.rs ./backend/

# Copy frontend build to project root (where build.rs expects it)
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

# Build native binary (Alpine uses musl by default)
WORKDIR /app/backend
# Static link C runtime for portability
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release && \
    cp target/release/home-server-navigator /tmp/home-server-navigator

# Stage 3: Final minimal image
FROM alpine:3.19

# Install CA certificates
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN adduser -D -u 1000 hsn

# Copy binary
COPY --from=backend-builder /tmp/home-server-navigator /usr/local/bin/home-server-navigator

# Create data directory
RUN mkdir -p /data && chown hsn:hsn /data

# Data directory (will be mounted as volume)
VOLUME ["/data"]

# Expose port
EXPOSE 8080

# Run as non-root user
USER hsn

ENTRYPOINT ["/usr/local/bin/home-server-navigator"]
CMD ["--host", "0.0.0.0", "--port", "8080", "--data-file", "/data/services.json"]
