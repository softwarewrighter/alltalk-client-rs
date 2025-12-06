#!/bin/bash
# at-build.sh - Build AllTalk (runs at-0 through at-2)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  AllTalk - Full Build"
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
./at-0-setup-dirs.sh

echo ""
echo "[2/3] Creating Dockerfile..."
./at-1-create-dockerfile.sh

echo ""
echo "[3/3] Building Docker image..."
./at-2-build.sh

echo ""
echo "=========================================="
echo "  AllTalk build complete!"
echo "=========================================="
echo ""
echo "To start: ./at-6-run.sh"
echo "To test:  ./at-7-test.sh"
