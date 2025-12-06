#!/bin/bash
# tou-6-run.sh - Start Toucan
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/toucan"

export TOU_PORT="${TOU_PORT:-1721}"

echo "=== Starting Toucan on port ${TOU_PORT} ==="

cd "${WORK_DIR}"

if docker compose ps 2>/dev/null | grep -qE "(Up|running)"; then
    echo "Toucan already running:"
    docker compose ps
    exit 0
fi

docker compose up -d

echo ""
echo "Waiting for Toucan to become healthy..."
echo "(First run downloads models)"
for i in {1..120}; do
    sleep 5
    HEALTH=$(docker inspect --format='{{.State.Health.Status}}' toucan 2>/dev/null || echo "starting")
    echo "[$((i*5))s] toucan: $HEALTH"
    if [[ "$HEALTH" == "healthy" ]]; then
        break
    fi
done

echo ""
docker compose ps

echo ""
echo "=== Toucan ready ==="
echo "API: http://localhost:${TOU_PORT}"
