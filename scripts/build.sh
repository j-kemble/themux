#!/usr/bin/env bash
# themux build script
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_DIR"

MODE="${1:-debug}"

case "$MODE" in
    release|--release|-r)
        echo "Building release..."
        cargo build --release
        echo "Binary: target/release/themux-gui"
        echo "CLI:    target/release/themux"
        ;;
    debug|--debug|-d|"")
        echo "Building debug..."
        cargo build
        echo "Binary: target/debug/themux-gui"
        echo "CLI:    target/debug/themux"
        ;;
    app)
        echo "Building app only..."
        cargo build -p themux-app
        ;;
    cli)
        echo "Building CLI only..."
        cargo build -p themux-cli
        ;;
    daemon)
        echo "Building daemon only..."
        cd daemon/cmuxd-remote
        go build -o "$PROJECT_DIR/build/cmuxd-remote" ./cmd/cmuxd-remote/
        ;;
    *)
        echo "Usage: $0 [debug|release|app|cli|daemon]"
        exit 1
        ;;
esac
