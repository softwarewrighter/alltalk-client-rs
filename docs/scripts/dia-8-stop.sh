#!/bin/bash
# dia-8-stop.sh - Stop Dia
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/dia"

echo "=== Stopping Dia ==="

cd "${WORK_DIR}"
docker compose down --remove-orphans 2>/dev/null || true

echo "Dia stopped."
