#!/usr/bin/env bash
set -euo pipefail

# Build all component workspaces in release mode

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${SCRIPT_DIR}")"

echo "Building all components in release mode..."

# Build tts-spec component
echo "Building tts-spec..."
cd "${PROJECT_ROOT}/components/tts-spec"
cargo build --release

# Build tts-cli component
echo "Building tts-cli..."
cd "${PROJECT_ROOT}/components/tts-cli"
cargo build --release

echo "Build complete."
