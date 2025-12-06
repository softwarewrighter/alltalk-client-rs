#!/bin/bash
# at-all.sh - Complete AllTalk pipeline: create, build, run, test
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== AllTalk Complete Pipeline ==="
echo ""

# Stop any existing containers
echo ">>> Stopping existing containers..."
"${SCRIPT_DIR}/at-8-stop.sh" 2>/dev/null || true

# Generate all Dockerfiles and server code
echo ""
echo ">>> Generating Dockerfiles and server code..."
"${SCRIPT_DIR}/at-1-create-dockerfile.sh"

# Build all images (includes base if needed)
echo ""
echo ">>> Building Docker images..."
"${SCRIPT_DIR}/at-2-build.sh"

# Start the stack
echo ""
echo ">>> Starting AllTalk stack..."
"${SCRIPT_DIR}/at-6-run.sh"

# Run tests
echo ""
echo ">>> Running tests..."
"${SCRIPT_DIR}/at-7-test.sh"

echo ""
echo "=== AllTalk Pipeline Complete ==="
