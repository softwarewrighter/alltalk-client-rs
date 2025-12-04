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

# Build tts-web component (server + WASM UI)
echo "Building tts-web..."
cd "${PROJECT_ROOT}/components/tts-web"

# Build the server
echo "  Building tts-web-server..."
cargo build --release -p tts-web-server

# Build WASM UI with Trunk
echo "  Building tts-web-ui (WASM)..."
cd "${PROJECT_ROOT}/components/tts-web/crates/tts-web-ui"
trunk build --release

echo "Build complete."
echo ""
echo "Artifacts:"
echo "  CLI:    ${PROJECT_ROOT}/components/tts-cli/target/release/ttsctl"
echo "  Server: ${PROJECT_ROOT}/components/tts-web/target/release/tts-web-server"
echo "  WebUI:  ${PROJECT_ROOT}/components/tts-web/crates/tts-web-ui/dist/"
