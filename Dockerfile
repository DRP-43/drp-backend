# Stage 1: Build
FROM rust:1.96-alpine AS build

# Install build dependencies for static linking
RUN apk add --no-cache musl-dev

WORKDIR /app

# Cache dependencies by building a dummy project first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source and rebuild
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Minimal production image
FROM scratch

# Copy CA certificates for HTTPS
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the compiled binary
# NOTE: The name is `drp-backend`. IF THE NAME CHANGES, CHANGE THIS vvv
COPY --from=build /app/target/release/drp-backend /server

ENV PORT=80
EXPOSE 80

ENTRYPOINT ["/server"]
