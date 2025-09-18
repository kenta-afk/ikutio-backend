FROM rust:1.89.0-alpine3.22

WORKDIR /app

RUN apk add --no-cache mold clang openssl-dev musl-dev openssl-libs-static

# Use mold as the linker for faster builds
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold"
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Copy migration files
COPY . migrations/

# Create database and run migrations
COPY <<EOF /app/run-migrations.sh
#!/bin/sh
set -e

echo "Starting migration process..."
echo "DATABASE_URL: \$DATABASE_URL"

echo "Creating database..."
if sqlx database create; then
    echo "Database created successfully"
else
    echo "Failed to create database or database already exists"
fi

echo "Running migrations..."
if sqlx migrate run; then
    echo "Migrations completed successfully"
else
    echo "Migration failed"
    exit 1
fi

echo "Migration process finished"
EOF

RUN chmod +x /app/run-migrations.sh

ENTRYPOINT ["/app/run-migrations.sh"]