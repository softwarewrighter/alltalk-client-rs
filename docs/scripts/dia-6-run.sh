#!/bin/bash
# dia-6-run.sh - Start Dia
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/dia"

export DIA_PORT="${DIA_PORT:-1110}"

echo "=== Starting Dia on port ${DIA_PORT} ==="

cd "${WORK_DIR}"

if docker compose ps 2>/dev/null | grep -qE "(Up|running)"; then
    echo "Dia already running:"
    docker compose ps
    exit 0
fi

docker compose up -d

echo ""
echo "Waiting for Dia to become healthy..."
echo "(First run downloads ~3GB model)"
for i in {1..120}; do
    sleep 5
    HEALTH=$(docker inspect --format='{{.State.Health.Status}}' dia 2>/dev/null || echo "starting")
    echo "[$((i*5))s] dia: $HEALTH"
    if [[ "$HEALTH" == "healthy" ]]; then
        break
    fi
done

echo ""
docker compose ps

echo ""
echo "=== Dia ready ==="
echo "API: http://localhost:${DIA_PORT}"
