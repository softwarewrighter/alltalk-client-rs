#!/usr/bin/env bash
set -euo pipefail

# Run the ttsctl CLI with passed arguments

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${SCRIPT_DIR}")"

cd "${PROJECT_ROOT}"

# Build if binary doesn't exist
if [[ ! -f "${PROJECT_ROOT}/target/release/ttsctl" ]]; then
    echo "Binary not found, building..."
    cargo build --release
fi

exec "${PROJECT_ROOT}/target/release/ttsctl" "$@"
