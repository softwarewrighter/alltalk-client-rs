#!/bin/bash
# test-all.sh - Test all TTS backends
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  Testing All TTS Backends"
echo "=========================================="
echo ""

PASSED=0
FAILED=0

# Test AllTalk
echo "--- AllTalk ---"
if ./at-7-test.sh; then
    ((PASSED++))
else
    ((FAILED++))
fi
echo ""

# Test Dia
echo "--- Dia ---"
if ./dia-7-test.sh; then
    ((PASSED++))
else
    ((FAILED++))
fi
echo ""

# Test GPT-SoVITS
echo "--- GPT-SoVITS ---"
if ./gpt-7-test.sh; then
    ((PASSED++))
else
    ((FAILED++))
fi
echo ""

# Test Toucan
echo "--- Toucan ---"
if ./tou-7-test.sh; then
    ((PASSED++))
else
    ((FAILED++))
fi
echo ""

echo "=========================================="
echo "  Test Results"
echo "=========================================="
echo "  Passed: $PASSED / 4"
echo "  Failed: $FAILED / 4"
echo ""

if [[ $FAILED -gt 0 ]]; then
    exit 1
fi
