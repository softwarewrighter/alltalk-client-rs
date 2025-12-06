#!/bin/bash
# stop-all.sh - Stop all TTS backends
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  Stopping All TTS Backends"
echo "=========================================="
echo ""

./at-8-stop.sh
./dia-8-stop.sh
./gpt-8-stop.sh
./tou-8-stop.sh

echo ""
echo "All backends stopped."
