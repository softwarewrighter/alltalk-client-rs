#!/bin/bash
# dia-9-logs.sh - Show Dia logs
set -euo pipefail

LINES="${1:-100}"

echo "=== Dia Logs (last ${LINES} lines) ==="
docker logs dia --tail "${LINES}" 2>&1
