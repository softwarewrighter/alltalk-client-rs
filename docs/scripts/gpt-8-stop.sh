#!/bin/bash
# gpt-8-stop.sh - Stop GPT-SoVITS
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/gptsovits"

echo "=== Stopping GPT-SoVITS ==="

cd "${WORK_DIR}"
docker compose down --remove-orphans 2>/dev/null || true

echo "GPT-SoVITS stopped."
