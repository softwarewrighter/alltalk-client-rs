#!/bin/bash
# tou-8-stop.sh - Stop Toucan
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/toucan"

echo "=== Stopping Toucan ==="

cd "${WORK_DIR}"
docker compose down --remove-orphans 2>/dev/null || true

echo "Toucan stopped."
