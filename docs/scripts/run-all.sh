#!/bin/bash
# run-all.sh - Start all TTS backends
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

# Default ports
export AT_PORT="${AT_PORT:-5157}"
export DIA_PORT="${DIA_PORT:-1110}"
export GPT_PORT="${GPT_PORT:-6910}"
export TOU_PORT="${TOU_PORT:-1721}"

echo "=========================================="
echo "  Starting All TTS Backends"
echo "=========================================="
echo ""
echo "Ports:"
echo "  AllTalk:     ${AT_PORT}"
echo "  Dia:         ${DIA_PORT}"
echo "  GPT-SoVITS:  ${GPT_PORT}"
echo "  Toucan:      ${TOU_PORT}"
echo ""

# Start each backend
./at-6-run.sh
echo ""
./dia-6-run.sh
echo ""
./gpt-6-run.sh
echo ""
./tou-6-run.sh

echo ""
echo "=========================================="
echo "  All backends started!"
echo "=========================================="
echo ""
echo "APIs:"
echo "  AllTalk:     http://localhost:${AT_PORT}"
echo "  Dia:         http://localhost:${DIA_PORT}"
echo "  GPT-SoVITS:  http://localhost:${GPT_PORT}"
echo "  Toucan:      http://localhost:${TOU_PORT}"
echo ""
echo "To test: ./test-all.sh"
echo "To stop: ./stop-all.sh"
