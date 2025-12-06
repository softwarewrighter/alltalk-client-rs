#!/bin/bash
# gpt-7-test.sh - Test GPT-SoVITS (using official api_v2)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/gptsovits"

GPT_PORT="${GPT_PORT:-6910}"
API_URL="http://localhost:${GPT_PORT}"

echo "=== Testing GPT-SoVITS ==="
echo "API: ${API_URL}"
echo ""

mkdir -p "${WORK_DIR}/outputs"
mkdir -p "${WORK_DIR}/voices"

# Check if running (api_v2 responds to / with method not allowed or similar)
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "${API_URL}/" 2>/dev/null || echo "000")
if [[ "$HTTP_CODE" == "000" ]]; then
    echo "ERROR: GPT-SoVITS not running"
    echo "Start with: ./gpt-6-run.sh"
    exit 1
fi
echo "GPT-SoVITS is responding (HTTP $HTTP_CODE)."
echo ""

# Check for voice samples
VOICE_FILES=$(ls "${WORK_DIR}/voices/"*.wav 2>/dev/null || true)
if [[ -z "$VOICE_FILES" ]]; then
    echo "WARNING: No voice samples in ${WORK_DIR}/voices/"
    echo ""
    echo "GPT-SoVITS requires a reference audio file for voice cloning."
    echo "Copy a WAV file to: ${WORK_DIR}/voices/reference.wav"
    echo ""
    echo "Skipping generation test (requires voice sample)"
    exit 0
fi

# Get first voice file
VOICE_FILE=$(ls "${WORK_DIR}/voices/"*.wav 2>/dev/null | head -1)
VOICE_NAME=$(basename "$VOICE_FILE")
echo "Using reference voice: $VOICE_NAME"
echo ""

echo "Testing GPT-SoVITS generation..."
# api_v2.py /tts endpoint
response=$(curl -s -X POST "${API_URL}/tts" \
    -H "Content-Type: application/json" \
    -d "{
        \"text\": \"Hello, this is a test of GPT SoVITS voice cloning.\",
        \"text_lang\": \"en\",
        \"ref_audio_path\": \"/workspace/GPT-SoVITS/voices/${VOICE_NAME}\",
        \"prompt_text\": \"Hello, this is a reference.\",
        \"prompt_lang\": \"en\"
    }" \
    -o "${WORK_DIR}/outputs/test-gptsovits.wav" \
    -w "%{http_code}" 2>&1) || response="000"

if [[ "$response" == "200" ]] && [[ -s "${WORK_DIR}/outputs/test-gptsovits.wav" ]]; then
    size=$(stat -c%s "${WORK_DIR}/outputs/test-gptsovits.wav" 2>/dev/null || stat -f%z "${WORK_DIR}/outputs/test-gptsovits.wav" 2>/dev/null || echo "0")
    if [[ "$size" -gt 1000 ]]; then
        echo "  SUCCESS: test-gptsovits.wav ($size bytes)"
        exit 0
    fi
fi

echo "  FAILED: HTTP $response"
echo "Check logs: ./gpt-9-logs.sh"
exit 1
