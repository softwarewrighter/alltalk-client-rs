#!/bin/bash
# build-all.sh - Build all TTS backends (builds shared base image first)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  Building All TTS Backends"
echo "=========================================="
echo ""

# Parse arguments
BACKENDS="${1:-all}"

# Check if base image exists
check_base_image() {
    if ! docker image inspect tts-base:local &>/dev/null; then
        echo "Base image 'tts-base:local' not found."
        echo "Building base image first..."
        echo ""
        ./base-build.sh
        echo ""
    else
        echo "Base image 'tts-base:local' found."
        echo ""
    fi
}

case "$BACKENDS" in
    all)
        echo "Building: base, alltalk, dia, gptsovits, toucan"
        echo ""
        check_base_image
        echo ""
        ./at-build.sh
        echo ""
        ./dia-build.sh
        echo ""
        ./gpt-build.sh
        echo ""
        ./tou-build.sh
        ;;
    base)
        ./base-build.sh
        ;;
    at|alltalk)
        check_base_image
        ./at-build.sh
        ;;
    dia)
        check_base_image
        ./dia-build.sh
        ;;
    gpt|gptsovits)
        check_base_image
        ./gpt-build.sh
        ;;
    tou|toucan)
        check_base_image
        ./tou-build.sh
        ;;
    *)
        echo "Usage: $0 [backend]"
        echo ""
        echo "Backends:"
        echo "  all       - Build all (default)"
        echo "  base      - Base image only"
        echo "  at        - AllTalk (parler, piper, xtts)"
        echo "  dia       - Dia 1.6B"
        echo "  gpt       - GPT-SoVITS"
        echo "  tou       - Toucan"
        exit 1
        ;;
esac

echo ""
echo "=========================================="
echo "  Build complete!"
echo "=========================================="
echo ""
echo "To start all: ./run-all.sh"
echo "To test all:  ./test-all.sh"
