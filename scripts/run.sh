#!/usr/bin/env bash
# themux run script
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_DIR"

# Build if needed
if [ ! -f target/debug/themux-gui ]; then
    echo "Building..."
    cargo build
fi

echo "Launching themux..."
RUST_LOG="${RUST_LOG:-themux=info}" \
    cargo run -p themux-app
