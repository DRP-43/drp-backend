# Stage 1: Build
FROM rust:1.96-alpine AS builder

# Install build dependencies for static linking
RUN apk add --no-cache musl-dev openssl-libs-static libpq-dev

WORKDIR /app

# Create dummy source for dependency caching.
#
# We replace `src/bin/app/main.rs` and `src/lib.rs` with dummy files that do nothing, so that we can
# compile all the dependencies first in a layer. This means that the next layer *only* compiles the
# build, so we can fast-forward this layer as it will (usually) remain the same.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src/bin/app;
RUN echo "fn main() {}" > src/bin/app/main.rs;
RUN echo "pub fn dummy() {}" > src/lib.rs;
# NOTE: necessary so that rust can include these libraries in build.
RUN RUSTFLAGS="-lpgcommon -lpgport -lssl -lcrypto" cargo build --release;
RUN rm -rf src;

# Copy source and rebuild
#
# NOTE: For some reason the `touch` commands are required for this to build properly. Otherwise the
# container just exits right after running.
#
# TODO: Figure out why the `touch` commands are required!
COPY src ./src
RUN touch src/bin/app/main.rs;
RUN touch src/lib.rs;
RUN cargo build --release;

# Stage 2: Runtime - minimal Alpine base
FROM alpine

# Install runtime dependencies
RUN apk add --no-cache libpq ca-certificates

# Copy the binary
COPY --from=builder /app/target/release/app /app

# Expose port env-var & image port
ENV PORT=80
EXPOSE 80

# Run the binary
ENTRYPOINT ["/app"]
