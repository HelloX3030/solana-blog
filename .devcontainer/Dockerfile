# Use Ubuntu 22.04 LTS (stable, long-term support)
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && \
	apt-get install -y --no-install-recommends \
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
	bash \
	&& rm -rf /var/lib/apt/lists/*

# Set Solana version
ARG SOLANA_RELEASE=v1.18.12

# Install Solana
RUN curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Add Solana to PATH
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Set workspace
WORKDIR /workspace

# Default shell
CMD ["/bin/bash"]
