#!/bin/bash
# gpt-9-logs.sh - Show GPT-SoVITS logs
set -euo pipefail

LINES="${1:-100}"

echo "=== GPT-SoVITS Logs (last ${LINES} lines) ==="
docker logs gptsovits --tail "${LINES}" 2>&1
