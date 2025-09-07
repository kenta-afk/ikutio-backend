FROM rust:1.89.0-alpine3.22

WORKDIR /app

RUN apk add --no-chache mold clang openssl-dev musl-dev openssl-libs-static

# Use mold as the linker for faster builds
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold"
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres