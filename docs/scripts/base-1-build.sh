#!/bin/bash
# base-1-build.sh - Build the shared TTS base image
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/base"

echo "=== Building TTS Base Image ==="
echo "This creates the shared foundation for all TTS models."
echo "Build time: ~10-15 minutes (first time)"
echo ""

cd "${WORK_DIR}"
docker build -t tts-base:local . 2>&1 | tee build.log

echo ""
echo "=== Base image build complete ==="
docker images tts-base:local
