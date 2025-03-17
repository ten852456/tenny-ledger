FROM ubuntu:22.04 as builder

# Accept the REBUILD argument to force rebuilds
ARG REBUILD=1

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install Rust and other dependencies
# Note: We use Ubuntu 22.04 which has compatible GLIBC versions
# libpq-dev is required for PostgreSQL client libraries during build
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libleptonica-dev \
    libtesseract-dev \
    tesseract-ocr \
    libclang-dev \
    clang \
    pkg-config \
    ca-certificates \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app

# Copy the entire source code
COPY . .

# Create entrypoint script for handling secrets
RUN echo '#!/bin/bash\n\
# Load secrets into environment variables\n\
if [ -f /run/secrets/jwt_secret ]; then\n\
  export JWT_SECRET=$(cat /run/secrets/jwt_secret)\n\
  echo "Loaded JWT secret from secrets"\n\
else\n\
  echo "WARNING: JWT secret file not found in secrets"\n\
fi\n\
\n\
if [ -f /run/secrets/google_vision_api_key ]; then\n\
  export GOOGLE_VISION_API_KEY=$(cat /run/secrets/google_vision_api_key)\n\
  echo "Loaded Google Vision API key from secrets"\n\
else\n\
  echo "INFO: Google Vision API key not found in secrets - will use Tesseract OCR only"\n\
fi\n\
\n\
# Execute the command passed to docker run\n\
exec "$@"' > docker-entrypoint.sh && \
chmod +x docker-entrypoint.sh

# Build the application
RUN cargo build --release

# Use the same Ubuntu version for runtime
FROM ubuntu:22.04

# Install only the runtime dependencies
# libssl3 is required for OpenSSL in Ubuntu 22.04
# libpq5 is required for PostgreSQL client libraries at runtime
RUN apt-get update && apt-get install -y \
    libtesseract-dev \
    tesseract-ocr \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for detailed logging
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/tenny-ledger-backend /app/

# Copy the entrypoint script
COPY --from=builder /usr/src/app/docker-entrypoint.sh /app/

# Create uploads directory
RUN mkdir -p /app/uploads

# Run the entrypoint script
ENTRYPOINT ["/app/docker-entrypoint.sh"]

# Default command
CMD ["./tenny-ledger-backend"] 