#!/bin/bash
# tou-9-logs.sh - Show Toucan logs
set -euo pipefail

LINES="${1:-100}"

echo "=== Toucan Logs (last ${LINES} lines) ==="
docker logs toucan --tail "${LINES}" 2>&1
