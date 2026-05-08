# themux

A native Linux terminal multiplexer for AI coding agents — port of [cmux](https://github.com/manaflow-ai/cmux) functionality to Linux.

**Status:** Pre-alpha development

## What is themux?

themux combines a GPU-accelerated terminal emulator (Ghostty), in-app browser (WebKitGTK), vertical workspace tabs, split panes, notification system, and extensive scripting API into one native Linux application built with Rust and GTK4.

## Architecture

```
┌─────────────────────────────────────────────┐
│              GTK4 UI Layer                   │
│  Sidebar │ Split Panes │ Tab Bar │ Browser   │
│  Ghostty GTK Terminal Widget (GPU-accelerated)│
├─────────────────────────────────────────────┤
│           Rust Application Core              │
│  Workspace Manager │ Socket Server │ Config   │
│  Session Persist   │ Notifications  │ Agents  │
├─────────────────────────────────────────────┤
│           IPC / Protocols                    │
│  Unix Socket (JSON-RPC V2) — cmux-compatible │
├─────────────────────────────────────────────┤
│       Cross-Platform Components              │
│  cmuxd-remote (Go) │ CLI (Rust) │ Tests (Python)│
└─────────────────────────────────────────────┘
```

## Project Structure

```
themux/
├── crates/
│   ├── themux-core/       # Business logic: workspace model, config, session, layout
│   ├── themux-socket/     # Unix socket server, V2 JSON-RPC protocol, event bus
│   ├── themux-cli/        # CLI binary (workspace, surface, browser, vm, ssh, tmux compat)
│   ├── themux-agent/      # Agent hooks: Claude Code, Codex, OpenCode, Hermes
│   ├── themux-notify/     # OSC detection, desktop notification bridge
│   └── themux-app/        # GTK4 application binary
├── ui/                    # GTK4 UI definitions, CSS, layouts
├── ghostty/               # Ghostty terminal engine (git submodule)
├── daemon/                # cmuxd-remote Go daemon (cross-platform)
├── data/                  # Default config, JSON schemas
├── tests/                 # Python integration tests (protocol-compatible)
├── scripts/               # Build, setup, test scripts
└── docs/                  # Architecture, protocol docs, contributing
```

## Quick Start

### Prerequisites

- Rust 1.85+
- GTK4 development libraries
- WebKitGTK 6.0 development libraries
- Zig 0.14+ (for Ghostty build)
- Go 1.22+ (for cmuxd-remote)
- Python 3.10+ (for tests)

### Build

```bash
# Install dependencies (Fedora)
sudo dnf install gtk4-devel webkit2gtk4.1-devel libadwaita-devel \
                 zig go python3-devel

# Clone with submodules
git clone --recurse-submodules https://github.com/themux-app/themux.git
cd themux

# Setup (builds Ghostty, cmuxd-remote)
./scripts/setup.sh

# Build everything
make build

# Or: build just the CLI
cargo build --release -p themux-cli

# Or: build the GTK app
make build-app
```

### Run

```bash
# Launch the GUI
./scripts/run.sh

# Or: use the CLI directly
./target/release/themux new-workspace --cwd ~/projects/my-app
./target/release/themux list-workspaces
./target/release/themux send --workspace 1 "echo hello"
```

## Compatibility

themux targets protocol compatibility with cmux's V2 JSON-RPC socket API. The Python test suite from cmux's `tests_v2/` runs against themux to verify compatibility.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

Commercial licenses available for organizations that cannot comply with GPL.
