#!/bin/bash
# gpt-6-run.sh - Start GPT-SoVITS
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/gptsovits"

export GPT_PORT="${GPT_PORT:-6910}"

echo "=== Starting GPT-SoVITS on port ${GPT_PORT} ==="

cd "${WORK_DIR}"

if docker compose ps 2>/dev/null | grep -qE "(Up|running)"; then
    echo "GPT-SoVITS already running:"
    docker compose ps
    exit 0
fi

docker compose up -d

echo ""
echo "Waiting for GPT-SoVITS to become healthy..."
echo "(First run downloads models)"
for i in {1..120}; do
    sleep 5
    HEALTH=$(docker inspect --format='{{.State.Health.Status}}' gptsovits 2>/dev/null || echo "starting")
    echo "[$((i*5))s] gptsovits: $HEALTH"
    if [[ "$HEALTH" == "healthy" ]]; then
        break
    fi
done

echo ""
docker compose ps

echo ""
echo "=== GPT-SoVITS ready ==="
echo "API: http://localhost:${GPT_PORT}"
