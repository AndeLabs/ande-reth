# syntax=docker/dockerfile:1.4
# =============================================================================
# PRODUCTION-OPTIMIZED EV-RETH DOCKERFILE
# Multi-arch support, security hardened, optimized for size and performance
# Target: ~80MB final image, sub-60s build time
# =============================================================================

ARG RUST_VERSION=1.88
ARG DEBIAN_VERSION=bookworm

# =============================================================================
# BUILDER STAGE: Optimized Rust build environment
# =============================================================================
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION} AS builder

# Build arguments
ARG TARGETOS
ARG TARGETARCH
ARG BUILD_PROFILE=release
ARG FEATURES="jemalloc asm-keccak"

# Set working directory
WORKDIR /build

# Metadata labels
LABEL stage=builder
LABEL org.opencontainers.image.source="https://github.com/ande-labs/ev-reth"

# Install build dependencies in a single layer
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libssl-dev \
        clang \
        libclang-dev \
        git \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Configure Rust for optimized builds with edition 2024 support (sin flags incompatibles)
ENV RUSTFLAGS="-C codegen-units=1 -C opt-level=3"
ENV CARGO_BUILD_JOBS=8
ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=10
ENV RUST_BACKTRACE=0

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY bin ./bin
COPY crates ./crates
COPY benches ./benches

# Build optimized release binary
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build \
        --profile ${BUILD_PROFILE} \
        --bin ev-reth \
        --features "${FEATURES}" \
    && cp target/${BUILD_PROFILE}/ev-reth /ev-reth

# Strip binary and verify
RUN strip --strip-all /ev-reth && \
    chmod +x /ev-reth && \
    /ev-reth --version

# =============================================================================
# RUNTIME STAGE: Minimal distroless image for security
# =============================================================================
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime

# Metadata for production image
LABEL org.opencontainers.image.title="EV-RETH"
LABEL org.opencontainers.image.description="ANDE EV-RETH Execution Client - Production Ready"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.vendor="ANDE Labs"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
LABEL org.opencontainers.image.url="https://github.com/ande-labs/ev-reth"
LABEL org.opencontainers.image.documentation="https://github.com/ande-labs/ev-reth/blob/main/README.md"

# Copy binary with proper ownership
COPY --from=builder --chown=nonroot:nonroot /ev-reth /usr/local/bin/ev-reth

# Create data directory
USER nonroot:nonroot
WORKDIR /data

# Network and API ports
EXPOSE 30303 30303/udp 8545 8546 8551 9001

# Volume for persistent data
VOLUME ["/data"]

# Production health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=120s --retries=3 \
    CMD ["/usr/local/bin/ev-reth", "--version"] || exit 1

# Default configuration
ENTRYPOINT ["/usr/local/bin/ev-reth"]
CMD ["node", \
     "--datadir", "/data", \
     "--http", \
     "--http.addr", "0.0.0.0", \
     "--http.port", "8545", \
     "--ws", \
     "--ws.addr", "0.0.0.0", \
     "--ws.port", "8546", \
     "--authrpc.addr", "0.0.0.0", \
     "--authrpc.port", "8551", \
     "--metrics", "0.0.0.0:9001"]

# Build info
ARG BUILD_DATE
ARG VCS_REF
LABEL org.opencontainers.image.created=$BUILD_DATE
LABEL org.opencontainers.image.revision=$VCS_REF