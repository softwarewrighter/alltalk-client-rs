#!/bin/bash
# base-build.sh - Build the shared TTS base image
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  TTS Base Image - Full Build"
echo "=========================================="
echo ""
echo "This creates the shared foundation:"
echo "  - Ubuntu 22.04 + CUDA 12.8"
echo "  - Python 3.11 + uv"
echo "  - PyTorch with CUDA"
echo "  - Common TTS dependencies"
echo ""

echo "[1/2] Creating Dockerfile..."
./base-0-create-dockerfile.sh

echo ""
echo "[2/2] Building Docker image..."
./base-1-build.sh

echo ""
echo "=========================================="
echo "  Base image ready: tts-base:local"
echo "=========================================="
echo ""
echo "Now build individual TTS models:"
echo "  ./at-build.sh   - AllTalk (parler, piper, xtts)"
echo "  ./dia-build.sh  - Dia 1.6B"
echo "  ./gpt-build.sh  - GPT-SoVITS"
echo "  ./tou-build.sh  - Toucan"
