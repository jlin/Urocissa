#!/bin/bash

# Files to remove
files=(
    "./gallery-backend/.env"
    "./gallery-backend/Rocket.toml"
    "./gallery-frontend/config.ts"
    "./*.log" # Match all .log files at the root level
)

# Folders to remove
folders=(
    "./gallery-backend/db"
    "./gallery-backend/object"
    "./gallery-backend/target"
    "./gallery-backend/upload"
)

# Remove specified files
echo "Removing specified files..."
for file in "${files[@]}"; do
    for f in $file; do # Loop over matched files (in case of wildcards)
        if [ -f "$f" ]; then
            rm "$f"
            echo "Removed: $f"
        else
            echo "File not found: $f"
        fi
    done
done

# Remove specified folders and their contents
echo "Removing specified folders and their contents..."
for folder in "${folders[@]}"; do
    if [ -d "$folder" ]; then
        rm -rf "$folder"
        echo "Removed folder: $folder"
    else
        echo "Folder not found: $folder"
    fi
done

# Remove specific Docker containers (excluding Buildx container)
echo "Removing Docker containers (excluding Buildx container)..."
for container in $(docker ps -aq); do
    if docker inspect "$container" 2>/dev/null | grep -q 'buildx_buildkit_multiarch-builder0'; then
        echo "Skipping Buildx container: $container"
    else
        docker rm -f "$container" 2>/dev/null && echo "Removed container: $container"
    fi
done

# Remove specific Docker images (excluding Buildx image)
echo "Removing Docker images (excluding Buildx image)..."
for image in $(docker images -q); do
    if docker inspect "$image" 2>/dev/null | grep -q 'moby/buildkit:buildx-stable-1'; then
        echo "Skipping Buildx image: $image"
    else
        docker rmi -f "$image" 2>/dev/null && echo "Removed image: $image"
    fi
done

echo "Cleanup completed."
