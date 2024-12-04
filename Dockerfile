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

# Define a dynamic directory path with an environment variable
ARG UROCISSA_PATH
ENV UROCISSA_PATH=${UROCISSA_PATH}

# Validate if UROCISSA_PATH is set
RUN if [ -z "${UROCISSA_PATH}" ]; then echo "UROCISSA_PATH is not set! Build failed." && exit 1; fi

# Clone the 'feat/docker' branch of the repository into the dynamic path
RUN git clone https://github.com/hsa00000/Urocissa ${UROCISSA_PATH}

# Set working directory
WORKDIR ${UROCISSA_PATH}

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set environment variable for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy your project files into the dynamic path
COPY . ${UROCISSA_PATH}

# Install Urocissa dependencies or perform setup
RUN node install-urocissa.mjs

# Default command
CMD ["node", "./run-urocissa.mjs"]
