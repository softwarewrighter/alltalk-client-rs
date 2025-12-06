#!/bin/bash
# at-9-logs.sh - Show AllTalk stack logs
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/alltalk"

SERVICE="${1:-}"
LINES="${2:-100}"

cd "${WORK_DIR}"

if [[ -n "$SERVICE" ]]; then
    echo "=== Logs for ${SERVICE} (last ${LINES} lines) ==="
    docker compose logs --tail "${LINES}" "$SERVICE" 2>&1
else
    echo "=== AllTalk Stack Logs (last ${LINES} lines per service) ==="
    echo "Usage: $0 [service] [lines]"
    echo "Services: at-gateway, at-parler, at-piper, at-xtts"
    echo ""
    docker compose logs --tail "${LINES}" 2>&1
fi
