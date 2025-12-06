#!/bin/bash
# tou-build.sh - Build Toucan (runs tou-0 through tou-2)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  Toucan - Full Build"
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
./tou-0-setup-dirs.sh

echo ""
echo "[2/3] Creating Dockerfile..."
./tou-1-create-dockerfile.sh

echo ""
echo "[3/3] Building Docker image..."
./tou-2-build.sh

echo ""
echo "=========================================="
echo "  Toucan build complete!"
echo "=========================================="
echo ""
echo "To start: ./tou-6-run.sh"
echo "To test:  ./tou-7-test.sh"
