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
RUN curl -sSfL https://static.rust-lang.org/dist/rust-1.79.0-x86_64-unknown-linux-gnu.tar.gz \
    | tar -xz -C /tmp \
 	&& /tmp/rust-1.79.0-x86_64-unknown-linux-gnu/install.sh --prefix=/usr/local \
 	&& rm -rf /tmp/rust-1.79.0-x86_64-unknown-linux-gnu

# Install Solana CLI
RUN curl -sSfL https://github.com/solana-labs/solana/releases/download/v1.18.12/solana-release-x86_64-unknown-linux-gnu.tar.bz2   | tar -xj -C /usr/local --strip-components=1

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli --locked --root /usr/local

WORKDIR /workspace
