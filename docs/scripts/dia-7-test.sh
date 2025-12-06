#!/bin/bash
# dia-7-test.sh - Test Dia
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/dia"

DIA_PORT="${DIA_PORT:-1110}"
API_URL="http://localhost:${DIA_PORT}"

echo "=== Testing Dia ==="
echo "API: ${API_URL}"
echo ""

mkdir -p "${WORK_DIR}/outputs"

# Check if running
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "${API_URL}/health" 2>/dev/null || echo "000")
if [[ "$HTTP_CODE" != "200" ]]; then
    echo "ERROR: Dia not running (HTTP $HTTP_CODE)"
    echo "Start with: ./dia-6-run.sh"
    exit 1
fi
echo "Dia is ready."
echo ""

echo "Testing Dia generation..."
response=$(curl -s -X POST "${API_URL}/api/tts-generate" \
    -F "text_input=[S1] Hello, this is a test of Dia dialogue synthesis. [S2] Yes, it sounds great!" \
    -o "${WORK_DIR}/outputs/test-dia.wav" \
    -w "%{http_code}" 2>&1) || response="000"

if [[ "$response" == "200" ]] && [[ -s "${WORK_DIR}/outputs/test-dia.wav" ]]; then
    size=$(stat -c%s "${WORK_DIR}/outputs/test-dia.wav" 2>/dev/null || stat -f%z "${WORK_DIR}/outputs/test-dia.wav" 2>/dev/null || echo "0")
    if [[ "$size" -gt 1000 ]]; then
        echo "  SUCCESS: test-dia.wav ($size bytes)"
        exit 0
    fi
fi

echo "  FAILED: HTTP $response"
echo "Check logs: ./dia-9-logs.sh"
exit 1
