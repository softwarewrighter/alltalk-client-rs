#!/bin/bash
# tou-2-build.sh - Build Toucan Docker image
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/toucan"

echo "=== Building Toucan Docker image ==="
echo "This may take several minutes..."

cd "${WORK_DIR}"
docker build -t toucan:local . 2>&1 | tee build.log

echo ""
echo "=== Toucan build complete ==="
docker images toucan:local
