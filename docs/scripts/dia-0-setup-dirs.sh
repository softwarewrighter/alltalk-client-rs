#!/bin/bash
# dia-0-setup-dirs.sh - Setup directories for Dia
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/dia"

echo "=== Setting up Dia directories ==="

mkdir -p "${WORK_DIR}/models"
mkdir -p "${WORK_DIR}/voices"
mkdir -p "${WORK_DIR}/outputs"

echo ""
echo "Dia directory structure:"
echo "  ${WORK_DIR}/"
echo "  ├── models/    # Dia models (auto-downloaded from HF)"
echo "  ├── voices/    # Reference audio (.wav) + transcripts (.txt)"
echo "  └── outputs/   # Generated audio files"
