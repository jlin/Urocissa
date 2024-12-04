#!/bin/bash

# Get the absolute path of this script
SCRIPT_DIR=$(dirname "$(realpath "$0")")

# Set the UROCISSA_PATH to the script's absolute path
UROCISSA_PATH="$SCRIPT_DIR"

# Set the path of the .env file
ENV_FILE="./gallery-backend/.env"
TEMP_ENV_FILE="./gallery-backend/temp.env"

# Initialize variables
PREDEFINED_VOLUMES=(
    "./gallery-backend/db:${UROCISSA_PATH}/gallery-backend/db"
    "./gallery-backend/object:${UROCISSA_PATH}/gallery-backend/object"
    "./gallery-backend/Rocket.toml:${UROCISSA_PATH}/gallery-backend/Rocket.toml"
)

if [[ -f "./gallery-frontend/config.ts" ]]; then
    PREDEFINED_VOLUMES+=("./gallery-frontend/config.ts:${UROCISSA_PATH}/gallery-frontend/config.ts")
fi

DYNAMIC_VOLUMES=()

# Check if the .env file exists
if [[ -f "$ENV_FILE" ]]; then
    echo "Found $ENV_FILE. Processing SYNC_PATH for dynamic volume mounts."

    # Read SYNC_PATH from the .env file
    SYNC_PATH=$(grep -E '^SYNC_PATH\s*=\s*' "$ENV_FILE" | sed 's/^SYNC_PATH\s*=\s*//')

    # Check if SYNC_PATH was read
    if [[ -n "$SYNC_PATH" ]]; then
        # If the value has quotes, remove them
        SYNC_PATH="${SYNC_PATH%\"}"
        SYNC_PATH="${SYNC_PATH#\"}"

        echo "Original SYNC_PATH is: $SYNC_PATH"

        # Split SYNC_PATH by commas and convert to an array
        IFS=',' read -ra PATHS <<< "$SYNC_PATH"

        ABS_PATHS=()

        # Get the directory where the .env file is located
        ENV_DIR=$(dirname "$ENV_FILE")

        for path in "${PATHS[@]}"; do
            # Remove leading and trailing spaces
            path=$(echo "$path" | xargs)

            # Check if the path is an absolute path
            if [[ "$path" = /* ]]; then
                abs_path="$path"
            else
                # Use realpath to convert relative path to absolute path based on ENV_DIR
                if command -v realpath &> /dev/null; then
                    abs_path=$(realpath -m "$ENV_DIR/$path")
                else
                    # If realpath does not exist, use another method
                    abs_path="$(cd "$ENV_DIR/$path" 2>/dev/null && pwd)"
                    if [[ -z "$abs_path" ]]; then
                        echo "Warning: Unable to resolve path $path"
                        abs_path="$ENV_DIR/$path"
                    fi
                fi
            fi

            ABS_PATHS+=("$abs_path")
        done

        # Convert the absolute path array to a comma-separated string
        ABS_SYNC_PATH=$(IFS=, ; echo "${ABS_PATHS[*]}")

        echo "Absolute SYNC_PATH is: $ABS_SYNC_PATH"

        # Prepare formatted dynamic volume mount output
        for abs_path in "${ABS_PATHS[@]}"; do
            # Use the original path as both the host and container path if it is absolute
            DYNAMIC_VOLUMES+=("$abs_path:$abs_path")
        done

        # Create a temporary .env file with the updated SYNC_PATH
        cp "$ENV_FILE" "$TEMP_ENV_FILE"
        sed -i "s|^SYNC_PATH\s*=.*|SYNC_PATH=$ABS_SYNC_PATH|" "$TEMP_ENV_FILE"
        PREDEFINED_VOLUMES+=("$TEMP_ENV_FILE:${UROCISSA_PATH}/gallery-backend/.env")
    else
        echo "Warning: SYNC_PATH variable not found or is empty in $ENV_FILE. Skipping dynamic volume mounts."
    fi
else
    echo "Warning: File $ENV_FILE not found. Proceeding without dynamic volume mounts."
fi

# Build the Docker image with UROCISSA_PATH as a build argument
echo "Building Docker image with UROCISSA_PATH set to $UROCISSA_PATH"
sudo docker build --build-arg UROCISSA_PATH=$UROCISSA_PATH -t urocissa .

# Prepare formatted predefined volume mount output
PREDEFINED_VOLUME_OUTPUT=""
for vol in "${PREDEFINED_VOLUMES[@]}"; do
    PREDEFINED_VOLUME_OUTPUT+=" \\
    -v \"$vol\""
done

# Prepare formatted dynamic volume mount output
DYNAMIC_VOLUME_OUTPUT=""
for vol in "${DYNAMIC_VOLUMES[@]}"; do
    DYNAMIC_VOLUME_OUTPUT+=" \\
    -v \"$vol\""
done

# Read port from Rocket.toml
ROCKET_PORT=$(grep -E '^port\s*=\s*' ./gallery-backend/Rocket.toml | sed 's/^port\s*=\s*//')

# If port not found, use default port 4000
if [[ -z "$ROCKET_PORT" ]]; then
    ROCKET_PORT=4000
    echo "Port not found in Rocket.toml. Using default port 4000."
fi

# Final Docker Run command
DOCKER_RUN_COMMAND="sudo docker run -it --rm \\
${PREDEFINED_VOLUME_OUTPUT} \\
${DYNAMIC_VOLUME_OUTPUT} \\
    -p ${ROCKET_PORT}:${ROCKET_PORT} urocissa"

# Output the final Docker Run command
echo -e "\nGenerated Docker Run command:\n"
echo "$DOCKER_RUN_COMMAND"

# Execute the Docker Run command immediately
echo -e "\nExecuting Docker Run command...\n"
eval "$DOCKER_RUN_COMMAND"

# Check if the Docker Run command was successful
if [[ $? -ne 0 ]]; then
    echo "Error: Docker Run command failed to execute"
    exit 1
else
    echo "Docker container has been successfully started"
fi
