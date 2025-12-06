#!/bin/bash
# at-8-stop.sh - Stop AllTalk stack
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/alltalk"

echo "=== Stopping AllTalk Stack ==="

cd "${WORK_DIR}"
docker compose down --remove-orphans 2>/dev/null || true

echo "AllTalk stack stopped (gateway, parler, piper, xtts)."
