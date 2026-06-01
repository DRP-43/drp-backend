# Stage 1: Build
FROM rust:1.96-alpine AS builder

# Install build dependencies for static linking
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source for dependency caching.
#
# We replace `src/bin/app/main.rs` and `src/lib.rs` with dummy files that do nothing, so that we can
# compile all the dependencies first in a layer. This means that the next layer *only* compiles the
# build, so we can fast-forward this layer as it will (usually) remain the same.
RUN mkdir -p src/bin/app && \
    echo "fn main() {}" > src/bin/app/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy source and rebuild
COPY . ./
RUN cargo build --release

# Stage 2: Runtime - completely empty base
FROM scratch

# Copy CA certificates for HTTPS
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the static binary
COPY --from=builder /app/target/release/app /app

# Expose port env-var & image port
ENV PORT=80
EXPOSE 80

# Run the binary
ENTRYPOINT ["/app"]
