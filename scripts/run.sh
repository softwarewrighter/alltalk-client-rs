#!/usr/bin/env bash
set -euo pipefail

# Run the ttsctl CLI with passed arguments

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${SCRIPT_DIR}")"
CLI_DIR="${PROJECT_ROOT}/components/tts-cli"

# Build if binary doesn't exist
if [[ ! -f "${CLI_DIR}/target/release/ttsctl" ]]; then
    echo "Binary not found, building..."
    cd "${CLI_DIR}"
    cargo build --release
fi

exec "${CLI_DIR}/target/release/ttsctl" "$@"
