FROM rust:1.96-alpine3.23

# Create and change to the app directory.
WORKDIR /app

# Copy over the app
COPY . ./

# Build the app
RUN cargo build --release

# Expose the ports
EXPOSE 8000

CMD ["./target/release/drp-backend"]
