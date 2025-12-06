#!/bin/bash
# tou-0-setup-dirs.sh - Setup directories for Toucan
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/toucan"

echo "=== Setting up Toucan directories ==="

mkdir -p "${WORK_DIR}/models"
mkdir -p "${WORK_DIR}/voices"
mkdir -p "${WORK_DIR}/outputs"

echo ""
echo "Toucan directory structure:"
echo "  ${WORK_DIR}/"
echo "  ├── models/    # Toucan models (auto-downloaded)"
echo "  ├── voices/    # Reference audio (.wav)"
echo "  └── outputs/   # Generated audio files"
