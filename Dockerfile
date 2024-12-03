# Use a base Linux image
FROM ubuntu:latest

# Install necessary dependencies
RUN apt update && apt install -y \
    git \
    curl \
    ffmpeg \
    npm \
    pkg-config \
    nodejs \
    && apt clean

# Clone the 'feat/docker' branch of the repository
RUN git clone -b feat/docker https://github.com/hsa00000/Urocissa /Urocissa

# Set working directory
WORKDIR /Urocissa

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set environment variable for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Urocissa dependencies or perform setup
RUN node install-urocissa.mjs

# Default command
CMD ["node", "./gallery-backend/run-urocissa.mjs"]
