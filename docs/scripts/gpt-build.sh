#!/bin/bash
# gpt-build.sh - Build GPT-SoVITS (runs gpt-0 through gpt-2)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  GPT-SoVITS - Full Build"
echo "=========================================="
echo ""

# Check if base image exists, build if not
if ! docker image inspect tts-base:local &>/dev/null; then
    echo "Base image 'tts-base:local' not found."
    echo "Building base image first..."
    echo ""
    ./base-build.sh
    echo ""
fi

echo "[1/3] Setting up directories..."
./gpt-0-setup-dirs.sh

echo ""
echo "[2/3] Creating Dockerfile..."
./gpt-1-create-dockerfile.sh

echo ""
echo "[3/3] Building Docker image..."
./gpt-2-build.sh

echo ""
echo "=========================================="
echo "  GPT-SoVITS build complete!"
echo "=========================================="
echo ""
echo "To start: ./gpt-6-run.sh"
echo "To test:  ./gpt-7-test.sh"
