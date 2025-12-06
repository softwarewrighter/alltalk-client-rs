#!/bin/bash
# tou-7-test.sh - Test Toucan
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/toucan"

TOU_PORT="${TOU_PORT:-1721}"
API_URL="http://localhost:${TOU_PORT}"

echo "=== Testing Toucan ==="
echo "API: ${API_URL}"
echo ""

mkdir -p "${WORK_DIR}/outputs"

# Check if running
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "${API_URL}/health" 2>/dev/null || echo "000")
if [[ "$HTTP_CODE" != "200" ]]; then
    echo "ERROR: Toucan not running (HTTP $HTTP_CODE)"
    echo "Start with: ./tou-6-run.sh"
    exit 1
fi
echo "Toucan is ready."
echo ""

echo "Testing Toucan generation..."
response=$(curl -s -X POST "${API_URL}/api/tts-generate" \
    -d "text_input=Hello, this is a test of Toucan multilingual text to speech." \
    -d "language=eng" \
    -o "${WORK_DIR}/outputs/test-toucan.wav" \
    -w "%{http_code}" 2>&1) || response="000"

if [[ "$response" == "200" ]] && [[ -s "${WORK_DIR}/outputs/test-toucan.wav" ]]; then
    size=$(stat -c%s "${WORK_DIR}/outputs/test-toucan.wav" 2>/dev/null || stat -f%z "${WORK_DIR}/outputs/test-toucan.wav" 2>/dev/null || echo "0")
    if [[ "$size" -gt 1000 ]]; then
        echo "  SUCCESS: test-toucan.wav ($size bytes)"
        exit 0
    fi
fi

echo "  FAILED: HTTP $response"
echo "Check logs: ./tou-9-logs.sh"
exit 1
