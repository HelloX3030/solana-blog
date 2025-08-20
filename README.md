# solana-blog

=> Solution?

# Set Solana version

ARG SOLANA_RELEASE=v1.18.12

# Install Solana CLI

RUN SOLANA_RELEASE=${SOLANA_RELEASE} \
 curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

=> set version via env, and than use the official fast installation script...
