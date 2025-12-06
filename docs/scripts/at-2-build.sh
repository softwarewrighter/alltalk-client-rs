#!/bin/bash
# at-2-build.sh - Build AllTalk Docker stack (gateway + parler + piper + xtts)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/alltalk"

echo "=== Building AllTalk Docker Stack ==="

# Ensure base image exists
if ! docker images tts-base:local --format "{{.Repository}}" | grep -q "tts-base"; then
    echo "Base image not found. Building tts-base:local first..."
    "${SCRIPT_DIR}/base-2-build.sh"
fi

echo ""
echo "Building AllTalk stack (gateway + parler + piper + xtts)..."
echo "This may take several minutes..."
echo ""

cd "${WORK_DIR}"

# Create voices directories if they don't exist
mkdir -p voices/parler voices/xtts

# Build all services using docker compose
docker compose build 2>&1 | tee build.log

echo ""
echo "=== AllTalk build complete ==="
echo ""
echo "Images built:"
docker images --filter "reference=at-*:local" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"
