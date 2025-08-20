# Use an official Debian-based image
FROM ubuntu:24.04

# Set non-interactive mode for apt
ENV DEBIAN_FRONTEND=noninteractive

# Install required dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        build-essential \
        pkg-config \
        libudev-dev \
        llvm \
        libclang-dev \
        protobuf-compiler \
        libssl-dev \
        git \
        ca-certificates \
        cmake \
        python3 \
        xz-utils \
        && rm -rf /var/lib/apt/lists/*

# -------------------
# Install Rust
# -------------------
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# -------------------
# Install Solana CLI
# -------------------
RUN sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# -------------------
# Install Anchor CLI via AVM
# -------------------
RUN cargo install --git https://github.com/coral-xyz/anchor avm \
    && avm install latest \
    && avm use latest

# -------------------
# Install Node.js (LTS) + Yarn
# -------------------
# Use NodeSource for latest maintained Node.js 20.x LTS
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && corepack enable \
    && corepack prepare yarn@stable --activate

# Verify installations
RUN rustc --version \
    && cargo --version \
    && solana --version \
    && anchor --version \
    && node --version \
    && npm --version \
    && yarn --version

# Set working directory
WORKDIR /workspace

# Default command
CMD [ "bash" ]
