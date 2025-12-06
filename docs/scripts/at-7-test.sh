#!/bin/bash
# at-7-test.sh - Test AllTalk engines
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/alltalk"

AT_PORT="${AT_PORT:-5157}"
API_URL="http://localhost:${AT_PORT}"

echo "=== Testing AllTalk ==="
echo "API: ${API_URL}"
echo ""

mkdir -p "${WORK_DIR}/outputs"

# Check if running and all backends are ready
READY_RESPONSE=$(curl -s "${API_URL}/api/ready" 2>/dev/null || echo '{"ready":false}')
READY_STATUS=$(echo "$READY_RESPONSE" | grep -o '"ready":\s*true' || true)
if [[ -z "$READY_STATUS" ]]; then
    echo "ERROR: AllTalk backends not ready"
    echo "Response: $READY_RESPONSE"
    echo "Start with: ./at-6-run.sh and wait for models to load"
    exit 1
fi
echo "AllTalk is ready."
echo ""

PASSED=0
FAILED=0

# Test Parler
echo "Testing Parler..."
response=$(curl -s -X POST "${API_URL}/api/tts-generate" \
    -d "text_input=Hello from Parler TTS." \
    -d "engine=parler" \
    -o "${WORK_DIR}/outputs/test-parler.wav" \
    -w "%{http_code}" 2>&1) || response="000"
if [[ "$response" == "200" ]] && [[ -s "${WORK_DIR}/outputs/test-parler.wav" ]]; then
    echo "  SUCCESS: test-parler.wav"
    PASSED=$((PASSED + 1))
else
    echo "  FAILED: HTTP $response"
    FAILED=$((FAILED + 1))
fi

# Test Piper
echo "Testing Piper..."
response=$(curl -s -X POST "${API_URL}/api/tts-generate" \
    -d "text_input=Hello from Piper TTS." \
    -d "engine=piper" \
    -o "${WORK_DIR}/outputs/test-piper.wav" \
    -w "%{http_code}" 2>&1) || response="000"
if [[ "$response" == "200" ]] && [[ -s "${WORK_DIR}/outputs/test-piper.wav" ]]; then
    echo "  SUCCESS: test-piper.wav"
    PASSED=$((PASSED + 1))
else
    echo "  FAILED: HTTP $response"
    FAILED=$((FAILED + 1))
fi

# Test XTTS
echo "Testing XTTS..."
response=$(curl -s -X POST "${API_URL}/api/tts-generate" \
    -d "text_input=Hello from XTTS voice cloning." \
    -d "engine=xtts" \
    -o "${WORK_DIR}/outputs/test-xtts.wav" \
    -w "%{http_code}" 2>&1) || response="000"
if [[ "$response" == "200" ]] && [[ -s "${WORK_DIR}/outputs/test-xtts.wav" ]]; then
    echo "  SUCCESS: test-xtts.wav"
    PASSED=$((PASSED + 1))
else
    echo "  FAILED: HTTP $response"
    FAILED=$((FAILED + 1))
fi

echo ""
echo "=== Results ==="
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"

if [[ $FAILED -gt 0 ]]; then
    echo ""
    echo "Check logs: ./at-9-logs.sh"
    exit 1
fi
