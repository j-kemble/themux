#!/usr/bin/env bash
# Build libghostty-vt and copy artifacts to the themux build directory.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
GHOSTTY_DIR="$PROJECT_DIR/ghostty"
OUTPUT_DIR="$PROJECT_DIR/build/libghostty"

echo "=== Building libghostty-vt ==="

# Build the library
cd "$GHOSTTY_DIR"
export PATH="$HOME/.local/bin:$PATH"
zig build -Demit-lib-vt -Doptimize=ReleaseFast

# Copy artifacts
mkdir -p "$OUTPUT_DIR/lib" "$OUTPUT_DIR/include"
cp -r zig-out/lib/libghostty-vt* "$OUTPUT_DIR/lib/"
cp -r zig-out/include/ghostty "$OUTPUT_DIR/include/"
cp zig-out/lib/pkgconfig/libghostty-vt.pc "$OUTPUT_DIR/" 2>/dev/null || true

echo "=== Build complete ==="
echo "Library: $OUTPUT_DIR/lib/libghostty-vt.so"
echo "Headers: $OUTPUT_DIR/include/ghostty/vt.h"
echo "Size: $(du -sh $OUTPUT_DIR | cut -f1)"
