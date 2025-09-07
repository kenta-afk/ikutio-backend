FROM rust:1.89.0-alpine3.22

WORKDIR /app

RUN apk add --no-cache mold clang curl

# Use mold as the linker for faster builds
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold"
RUN cargo install cargo-watch

COPY . .

EXPOSE 50053