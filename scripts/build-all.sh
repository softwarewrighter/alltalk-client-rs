#!/usr/bin/env bash
set -euo pipefail

# Build all workspace crates in release mode

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${SCRIPT_DIR}")"

cd "${PROJECT_ROOT}"

echo "Building all crates in release mode..."
cargo build --release

echo "Build complete."
