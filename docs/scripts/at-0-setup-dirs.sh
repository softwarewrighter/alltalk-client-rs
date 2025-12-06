#!/bin/bash
# at-0-setup-dirs.sh - Setup directories for AllTalk (parler, piper, xtts)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/alltalk"

echo "=== Setting up AllTalk directories ==="

# Create engine-specific subdirectories
mkdir -p "${WORK_DIR}/models/parler"
mkdir -p "${WORK_DIR}/models/piper"
mkdir -p "${WORK_DIR}/models/xtts"
mkdir -p "${WORK_DIR}/voices/parler"
mkdir -p "${WORK_DIR}/voices/piper"
mkdir -p "${WORK_DIR}/voices/xtts"
mkdir -p "${WORK_DIR}/outputs"

# Download Piper voice model if not exists
PIPER_MODEL="en_US-amy-medium"
if [[ ! -f "${WORK_DIR}/models/piper/${PIPER_MODEL}.onnx" ]]; then
    echo "Downloading Piper voice model: ${PIPER_MODEL}..."
    curl -L -o "${WORK_DIR}/models/piper/${PIPER_MODEL}.onnx" \
        "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/amy/medium/${PIPER_MODEL}.onnx"
    curl -L -o "${WORK_DIR}/models/piper/${PIPER_MODEL}.onnx.json" \
        "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/amy/medium/${PIPER_MODEL}.onnx.json"
    echo "Piper model downloaded."
else
    echo "Piper model already exists."
fi

echo ""
echo "AllTalk directory structure:"
echo "  ${WORK_DIR}/"
echo "  ├── models/"
echo "  │   ├── parler/    # Parler models (auto-downloaded)"
echo "  │   ├── piper/     # Piper .onnx voice models"
echo "  │   └── xtts/      # XTTS models (auto-downloaded)"
echo "  ├── voices/"
echo "  │   ├── parler/    # Parler voice descriptions"
echo "  │   ├── piper/     # Piper custom voices"
echo "  │   └── xtts/      # XTTS reference audio (.wav)"
echo "  └── outputs/       # Generated audio files"
