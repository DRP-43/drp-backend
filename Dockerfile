# Stage 1: Build
FROM rust:1.96-slim AS builder

# Install build dependencies for static linking
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev;
RUN rm -rf /var/lib/apt/lists/*;

WORKDIR /app

# Create dummy source for dependency caching.
#
# We replace `src/bin/app/main.rs` and `src/lib.rs` with dummy files that do nothing, so that we can
# compile all the dependencies first in a layer. This means that the next layer *only* compiles the
# build, so we can fast-forward this layer as it will (usually) remain the same.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src/bin/app;
RUN echo "fn main() {}" > src/bin/app.rs;
RUN echo "pub fn dummy() {}" > src/lib.rs;
RUN cargo build --release;
RUN rm -rf src;

# Copy source and rebuild
#
# NOTE: The `touch` commands are necessary as they update the modification timestamp on the files,
# which cargo uses to see what needs to be rebuilt. If these files aren't `touch`-ed and have their
# modification timestamp updated, then cargo will assume no change and release the dummy binary
# built in the previous step
COPY src ./src
RUN touch src/bin/app.rs;
RUN touch src/lib.rs;
RUN cargo build --release;

# Stage 2: Runtime - minimal Alpine base
FROM debian:trixie-slim

# Install runtime dependencies
RUN apt-get update;
RUN apt-get install -y libpq5 ca-certificates;
RUN rm -rf /var/lib/apt/lists/*;

# Copy the binary
COPY --from=builder /app/target/release/app /app

# Expose port env-var & image port
ENV PORT=80
EXPOSE 80

# Run the binary
ENTRYPOINT ["/app"]
