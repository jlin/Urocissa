#!/bin/bash

# Default settings
DEBUG=false
LOG_FILE=""
BUILD_TYPE="release"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --debug)
            DEBUG=true
            shift
            ;;
        --log-file)
            LOG_FILE="$2"
            shift 2
            ;;
        --build-type)
            BUILD_TYPE="$2"
            if [[ "$BUILD_TYPE" != "release" && "$BUILD_TYPE" != "debug" ]]; then
                echo "Error: Invalid build type. Use 'release' or 'debug'."
                exit 1
            fi
            shift 2
            ;;
        *)
            echo "Error: Unknown option $1"
            exit 1
            ;;
    esac
done

# Function to output debug information
debug_log() {
    local message="$1"
    if [[ "$DEBUG" == true ]]; then
        if [[ -n "$LOG_FILE" ]]; then
            echo "$message" >> "$LOG_FILE"
        else
            echo "$message"
        fi
    fi
}

# Get the absolute path of this script
SCRIPT_DIR=$(dirname "$(realpath "$0")")

# Set the UROCISSA_PATH to the script's absolute path
UROCISSA_PATH="$SCRIPT_DIR"

debug_log "Script directory set to $SCRIPT_DIR"
debug_log "Build type is set to $BUILD_TYPE"

# Set the path of the .env file
ENV_FILE="./gallery-backend/.env"
TEMP_ENV_FILE="./gallery-backend/temp.env"

# Initialize volumes array
PREDEFINED_VOLUMES=()
DYNAMIC_VOLUMES=()

# Function to ensure config files exist and add to volume mounts
ensure_config_file() {
    local source_file="$1"
    local target_file="$2"
    local volume_path="${3:-$target_file}"

    if [[ ! -f "$target_file" ]]; then
        debug_log "$target_file not found. Copying from $source_file."
        mv "$source_file" "$target_file"
        cp "$target_file" "$source_file"
    fi

    # Add to predefined volumes if a volume path is provided
    if [[ -n "$volume_path" ]]; then
        PREDEFINED_VOLUMES+=("$target_file:$volume_path")
    fi
}

# Ensure necessary config files exist and set up volume mounts
ensure_config_file "./gallery-backend/Rocket.default.toml" "./gallery-backend/Rocket.toml" "${UROCISSA_PATH}/gallery-backend/Rocket.toml"
ensure_config_file "./gallery-frontend/config.default.ts" "./gallery-frontend/config.ts" "${UROCISSA_PATH}/gallery-frontend/config.ts"
ensure_config_file "./gallery-backend/.env.default" "$ENV_FILE" "${UROCISSA_PATH}/gallery-backend/.env"

# Convert CRLF to LF in the .env file
sed -i 's/\r$//' "$ENV_FILE"

# Process SYNC_PATH for dynamic volume mounts
SYNC_PATH=$(grep -E '^SYNC_PATH\s*=\s*' "$ENV_FILE" | sed 's/^SYNC_PATH\s*=\s*//')

if [[ -n "$SYNC_PATH" ]]; then
    SYNC_PATH="${SYNC_PATH%\"}"
    SYNC_PATH="${SYNC_PATH#\"}"
    debug_log "Original SYNC_PATH is: $SYNC_PATH"

    IFS=',' read -ra PATHS <<< "$SYNC_PATH"

    for path in "${PATHS[@]}"; do
        trimmed_path=$(echo "$path" | xargs)
        abs_path=$(realpath -m "$(dirname "$ENV_FILE")/$trimmed_path")
        DYNAMIC_VOLUMES+=("$abs_path:$abs_path")
    done
else
    debug_log "Warning: SYNC_PATH variable not found or is empty in $ENV_FILE. Skipping dynamic volume mounts."
fi

PREDEFINED_VOLUMES+=(
    "./gallery-backend/db:${UROCISSA_PATH}/gallery-backend/db"
    "./gallery-backend/object:${UROCISSA_PATH}/gallery-backend/object"
)

debug_log "Predefined volumes: ${PREDEFINED_VOLUMES[*]}"
debug_log "Dynamic volumes: ${DYNAMIC_VOLUMES[*]}"

# Build the Docker image with UROCISSA_PATH and build type as build arguments
debug_log "Building Docker image with UROCISSA_PATH=$UROCISSA_PATH and BUILD_TYPE=$BUILD_TYPE"
DOCKER_BUILD_COMMAND="sudo docker build \
    --build-arg UROCISSA_PATH=${UROCISSA_PATH} \
    --build-arg BUILD_TYPE=${BUILD_TYPE} \
    -t urocissa ."
eval "$DOCKER_BUILD_COMMAND"

# Extract port from Rocket.toml
ROCKET_PORT=$(grep -E '^port\s*=\s*' ./gallery-backend/Rocket.toml | sed -E 's/^port\s*=\s*"?([0-9]+)"?/\1/' | tr -d '[:space:]')
ROCKET_PORT=${ROCKET_PORT:-4000}
debug_log "Using port: $ROCKET_PORT"

# Generate Docker Run command
DOCKER_RUN_COMMAND="docker run -it --rm"
for vol in "${PREDEFINED_VOLUMES[@]}"; do
    DOCKER_RUN_COMMAND+=" -v $vol"
done
for vol in "${DYNAMIC_VOLUMES[@]}"; do
    DOCKER_RUN_COMMAND+=" -v $vol"
done
DOCKER_RUN_COMMAND+=" -p ${ROCKET_PORT}:${ROCKET_PORT} urocissa"

# Output and execute the Docker Run command
debug_log "Generated Docker Run command: $DOCKER_RUN_COMMAND"
eval "$DOCKER_RUN_COMMAND"

# Check if Docker Run succeeded
if [[ $? -ne 0 ]]; then
    debug_log "Error: Docker Run command failed to execute"
    exit 1
else
    debug_log "Docker container has been successfully started"
fi
