#!/usr/bin/env bash
# themux test runner
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_DIR"

echo "=== Running Rust unit tests ==="
cargo test --workspace

echo ""
echo "=== Running Python integration tests ==="
if [ -d tests ] && [ -f tests/requirements.txt ]; then
    cd tests
    python3 -m pytest -v "$@"
else
    echo "No Python tests found."
fi
