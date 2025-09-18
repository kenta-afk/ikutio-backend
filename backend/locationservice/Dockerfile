FROM rust:1.89.0-alpine3.22 AS builder

WORKDIR /app

RUN apk add --no-cache mold clang curl

# Use mold as the linker for faster builds
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold"

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy proto and source files
COPY proto ./proto
COPY src ./src


# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:3.22

WORKDIR /app

RUN apk add --no-cache ca-certificates

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/locationservice .

EXPOSE 50055

CMD ["./locationservice"]