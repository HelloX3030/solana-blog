FROM ubuntu:22.04

# System dependencies
RUN apt-get update && apt-get install -y \
	curl \
	git \
	build-essential \
	pkg-config \
	libssl-dev \
	libudev-dev \
	python3 \
	python3-pip \
	ca-certificates \
	gnupg \
	&& rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.79.0
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default 1.79.0

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.12/install)" "" -y -b /usr/local/bin

# Set PATH for Rust + Solana (for Anchor CLI)
ENV PATH="/root/.cargo/bin:/root/.local/share/solana/install/active_release/bin:${PATH}"

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli --locked

WORKDIR /workspace
