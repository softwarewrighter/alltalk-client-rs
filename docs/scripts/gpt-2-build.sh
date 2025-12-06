#!/bin/bash
# gpt-2-build.sh - Pull official GPT-SoVITS Docker image
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/gptsovits"

echo "=== Pulling official GPT-SoVITS Docker image ==="
echo "Image: xxxxrt666/gpt-sovits:latest-cu128"
echo "This may take several minutes on first pull..."

docker pull xxxxrt666/gpt-sovits:latest-cu128 2>&1 | tee "${WORK_DIR}/pull.log"

echo ""
echo "=== GPT-SoVITS image ready ==="
docker images xxxxrt666/gpt-sovits:latest-cu128
