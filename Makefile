# Build everything
.PHONY: all build setup clean test lint fmt check

# Default target
all: build

# Setup submodules and dependencies
setup:
	./scripts/setup.sh

# Build all Rust crates
build:
	cargo build --release

# Build just the GTK app
build-app:
	cargo build --release -p themux-app
	cargo build --release -p themux-cli

# Build just the CLI
build-cli:
	cargo build --release -p themux-cli

# Run tests (Rust)
test:
	cargo test --workspace

# Run Python integration tests
test-integration:
	cd tests && python -m pytest -v

# Lint
lint:
	cargo clippy --workspace -- -D warnings

# Format
fmt:
	cargo fmt --all

# Type check
check:
	cargo check --workspace

# Clean build artifacts
clean:
	cargo clean
	rm -rf /build/

# Build cmuxd-remote Go daemon
build-daemon:
	cd daemon/cmuxd-remote && go build -o ../../build/cmuxd-remote ./cmd/cmuxd-remote/

# Build Ghostty (via zig)
build-ghostty:
	cd ghostty && zig build -Doptimize=ReleaseFast

# Development: watch for changes and rebuild
dev:
	cargo watch -x 'build -p themux-app'

# Run the app
run:
	cargo run --release -p themux-app

# Run with debug logging
run-debug:
	RUST_LOG=themux=debug cargo run -p themux-app

# Package as Flatpak
flatpak:
	flatpak-builder --user --install build-dir flatpak/org.themux.Themux.json --force-clean
