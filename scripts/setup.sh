#!/usr/bin/env bash
# themux setup script
# Initializes submodules, builds Ghostty, cmuxd-remote, and Rust workspace.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== themux setup ==="
echo "Project: $PROJECT_DIR"

# 1. Check prerequisites
echo ""
echo "--- Checking prerequisites ---"

check_cmd() {
    if command -v "$1" &>/dev/null; then
        echo "  ✓ $1: $(command -v "$1")"
    else
        echo "  ✗ $1: NOT FOUND (install it first)"
        MISSING=1
    fi
}

MISSING=0
check_cmd rustc
check_cmd cargo
check_cmd zig
check_cmd go
check_cmd python3
check_cmd pkg-config

if [ "$MISSING" = "1" ]; then
    echo ""
    echo "Error: missing prerequisites. On Fedora:"
    echo "  sudo dnf install rust cargo zig go python3-devel pkgconfig \\"
    echo "    gtk4-devel webkit2gtk4.1-devel libadwaita-devel"
    exit 1
fi

# 2. Initialize git submodules
echo ""
echo "--- Initializing submodules ---"
cd "$PROJECT_DIR"
if [ -f .gitmodules ]; then
    git submodule update --init --recursive
fi

# 3. Build Ghostty terminal engine
echo ""
echo "--- Building Ghostty ---"
if [ -d ghostty ] && [ -f ghostty/build.zig ]; then
    cd ghostty
    zig build -Doptimize=ReleaseFast || echo "  Warning: Ghostty build failed; continuing..."
    cd "$PROJECT_DIR"
else
    echo "  Ghostty submodule not found — add it with:"
    echo "    git submodule add https://github.com/ghostty-org/ghostty.git ghostty"
fi

# 4. Build cmuxd-remote Go daemon
echo ""
echo "--- Building cmuxd-remote ---"
if [ -d daemon/cmuxd-remote ]; then
    cd daemon/cmuxd-remote
    go build -o "$PROJECT_DIR/build/cmuxd-remote" ./cmd/cmuxd-remote/ 2>/dev/null || \
        echo "  Warning: cmuxd-remote build failed; continuing..."
    cd "$PROJECT_DIR"
else
    echo "  cmuxd-remote not found — add it with:"
    echo "    cp -r <cmux-repo>/daemon/remote daemon/cmuxd-remote"
fi

# 5. Build Rust workspace
echo ""
echo "--- Building Rust workspace ---"
cargo build --workspace

echo ""
echo "=== Setup complete ==="
echo "Run './scripts/run.sh' to launch the GUI."
echo "Run 'cargo build --release' for release build."
