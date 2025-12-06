#!/bin/bash
# at-6-run.sh - Start AllTalk stack (gateway + parler + piper + xtts)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/alltalk"

export AT_PORT="${AT_PORT:-5157}"

echo "=== Starting AllTalk Stack on port ${AT_PORT} ==="

cd "${WORK_DIR}"

if docker compose ps 2>/dev/null | grep -qE "(Up|running)"; then
    echo "AllTalk stack already running:"
    docker compose ps
    exit 0
fi

docker compose up -d

CONTAINERS=(at-gateway at-parler at-piper at-xtts)

echo ""
echo "Waiting for all containers to become healthy..."
echo ""

ALL_HEALTHY=false
for i in {1..60}; do
    sleep 5
    ALL_HEALTHY=true
    STATUS_LINE=""

    for container in "${CONTAINERS[@]}"; do
        HEALTH=$(docker inspect --format='{{.State.Health.Status}}' "$container" 2>/dev/null || echo "starting")
        STATUS_LINE="${STATUS_LINE} ${container}:${HEALTH}"
        if [[ "$HEALTH" != "healthy" ]]; then
            ALL_HEALTHY=false
        fi
    done

    echo "[$((i*5))s]${STATUS_LINE}"

    if $ALL_HEALTHY; then
        break
    fi
done

echo ""
docker compose ps

if ! $ALL_HEALTHY; then
    echo ""
    echo "ERROR: Containers failed to become healthy within 300s"
    echo "Check logs with: ./at-9-logs.sh"
    exit 1
fi

echo ""
echo "=== AllTalk Stack Ready ==="
echo "Gateway: http://localhost:${AT_PORT}"
echo "Engines: parler, piper, xtts"
echo ""
echo "Test commands:"
echo "  curl http://localhost:${AT_PORT}/health"
echo "  curl http://localhost:${AT_PORT}/api/ready"
echo "  curl http://localhost:${AT_PORT}/api/engines"
