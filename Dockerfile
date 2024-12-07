# Use the Alpine variant of cargo-chef as the base image for building
FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef

# Define the build type as a build argument (default to "release")
ARG BUILD_TYPE=release
# Set the build type as an environment variable
ENV BUILD_TYPE=${BUILD_TYPE}

# Set the working directory for the backend
WORKDIR /app/gallery-backend

# Planner stage: Generate the build recipe
FROM chef AS planner
# Copy the backend source code to the container
COPY ./gallery-backend /app/gallery-backend

# Generate the Cargo recipe file for reproducible builds
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage: Build the Rust backend
FROM chef AS builder
# Install necessary dependencies for building the Rust project
RUN apk add --no-cache \
    openssl-dev \
    openssl-libs-static

# Copy the generated recipe from the planner stage
COPY --from=planner /app/gallery-backend/recipe.json /app/gallery-backend/recipe.json

# Use the build argument to determine whether to build in release mode or debug mode
RUN if [ "${BUILD_TYPE}" = "release" ]; then \
        cargo chef cook --release --recipe-path recipe.json; \
    else \
        cargo chef cook --recipe-path recipe.json; \
    fi

# Copy the backend source code to the container
COPY ./gallery-backend /app/gallery-backend

# Build the backend binary based on the build type
RUN if [ "${BUILD_TYPE}" = "release" ]; then \
        cargo build --release --bin Urocissa; \
    else \
        cargo build --bin Urocissa; \
    fi

# Copy the built binary to a consistent directory for easier access
RUN mkdir -p /app/gallery-backend/bin && \
    cp /app/gallery-backend/target/${BUILD_TYPE}/Urocissa /app/gallery-backend/bin/Urocissa

# Frontend builder stage: Build the frontend assets
FROM node:lts-alpine AS frontend-builder

# Set the working directory for the frontend
WORKDIR /app/gallery-frontend

# Copy the frontend source code to the container
COPY ./gallery-frontend /app/gallery-frontend

# Build the frontend assets
RUN npm run build

# Runtime stage: Assemble the final runtime image
FROM alpine:latest AS runtime

# Install runtime dependencies (e.g., ffmpeg for media processing)
RUN apk add --no-cache ffmpeg

# Define a dynamic repository path using an argument and environment variable
ARG UROCISSA_PATH=/app
ENV UROCISSA_PATH=${UROCISSA_PATH}

# Ensure the working directory exists for the backend
WORKDIR ${UROCISSA_PATH}/gallery-backend

# Copy the backend binary from the builder stage to the runtime image
COPY --from=builder /app/gallery-backend/bin/Urocissa ${UROCISSA_PATH}/gallery-backend

# Copy the built frontend assets from the frontend-builder stage
COPY --from=frontend-builder /app/gallery-frontend/dist ${UROCISSA_PATH}/gallery-frontend/dist
COPY --from=frontend-builder /app/gallery-frontend/public ${UROCISSA_PATH}/gallery-frontend/public

# Copy the frontend configuration file to the runtime image
COPY ./gallery-frontend/config.ts ${UROCISSA_PATH}/gallery-frontend

# Copy the backend configuration files to the runtime image
COPY ./gallery-backend/.env ${UROCISSA_PATH}/gallery-backend
COPY ./gallery-backend/Rocket.toml ${UROCISSA_PATH}/gallery-backend

# Define the command to run the backend application
ENTRYPOINT [ "./Urocissa" ]
