#!/bin/bash
# dia-2-build.sh - Build Dia Docker image
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/dia"

echo "=== Building Dia Docker image ==="
echo "This may take several minutes..."

cd "${WORK_DIR}"
docker build -t dia:local . 2>&1 | tee build.log

echo ""
echo "=== Dia build complete ==="
docker images dia:local
