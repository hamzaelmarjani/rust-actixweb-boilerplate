# -------------------------- Docker File ---------------------------------
# Optimized Dockerfile to dockernize the Rust based server-app.
# 1. Create a new empty Cargo app
# 2. Copy our server-app content inside it
# 3. Run the app
# ------------------------------------------------------------------------
# I do this to ingore a large docker image due to Cargo dependencies
# and builded binary, this is ok if you want to run the app once,
# but in our case, this large app will take a huge space
# on the project storage on the cloud when deploy it,
# this is why i optimaze it the maximum i can.
# If you have a better way, you can share it with me on the github repo ✌️
# ------------------------------------------------------------------------

# Multi-stage build for optimized Rust container
FROM rust:latest AS builder

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests first (for better layer caching)
COPY Cargo.toml Cargo.lock .cargo/* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached unless Cargo.toml changes)
RUN cargo build --release --bin rust-actixweb-boilerplate && rm src/main.rs

# Copy source code
COPY src ./src
COPY templates ./templates

# Build the actual application
# Touch main.rs to ensure it's rebuilt
RUN touch src/main.rs && cargo build --release --bin rust-actixweb-boilerplate

# Runtime stage - use distroless for minimal size
FROM gcr.io/distroless/cc-debian12

# Create non-root user (distroless provides nonroot user)
USER nonroot:nonroot

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rust-actixweb-boilerplate /app/server

# Expose port
EXPOSE 8080

# Set working directory
WORKDIR /app

# Run the application
CMD ["./server"]