# Phase 0: Foundation

> **Status:** Phase complete | **Last updated:** 2026-05-08 | **Weeks:** 1-2 | **Goal:** Shell app with terminal, basic socket server, CLI compatibility

## Prerequisites

- [x] Rust toolchain installed (`rustc`, `cargo`)
- [x] GTK4, WebKitGTK, libadwaita dev packages installed
- [x] Zig installed (for Ghostty build)
- [ ] Go installed (for cmuxd-remote) — not needed until Phase 2
- [x] `./scripts/build-libghostty.sh` builds libghostty-vt successfully
- [x] Ghostty source vendored at `ghostty/` (libghostty-vt only, trimmed)
- [x] `cargo build --workspace` passes

## Deliverables

### 0.1 Build System

- [x] Cargo workspace compiles all 7 crates cleanly
- [ ] `cargo fmt --all -- --check` passes — not yet checked
- [ ] `cargo clippy --workspace -- -D warnings` passes — not yet checked
- [x] `cargo test --workspace` passes (empty tests)
- [x] Makefile targets work: `build`, `test`, `clean`, `run`
- [x] GitHub Actions CI created at `.github/workflows/ci.yml`

### 0.2 Ghostty Terminal Integration (libghostty-vt)

- [x] Ghostty source vendored at `ghostty/` (trimmed to libghostty-vt only)
- [x] libghostty-vt builds via `scripts/build-libghostty.sh`
- [x] Rust FFI bindings created at `crates/ghostty-sys/`
- [x] TerminalWidget wrapper exists in `themux-app/src/ui/terminal.rs`
- [ ] Terminal renders shell output in GTK window — needs PTY plumbing
- [ ] Terminal accepts keyboard input — needs event wiring
- [ ] Font rendering works — needs renderer integration
- [ ] Terminal supports scrollback — available in libghostty-vt API
- [ ] Terminal supports copy/paste — needs implementation
- [ ] Terminal resize works — `ghostty_terminal_resize` wired

### 0.3 Basic Socket Server

- [x] Unix socket listener starts on `~/.local/share/themux/themux.sock`
- [x] Socket auth (password mode) works
- [x] `system.ping` returns `{"pong": true}`
- [x] `system.identify` returns server info
- [x] `system.capabilities` returns feature list
- [x] Unknown methods return `method_not_found` error
- [x] Socket cleanup on shutdown (remove stale socket file)

### 0.4 Basic CLI

- [x] `themux ping` works via socket
- [x] `themux version` prints version
- [x] `themux capabilities` lists features
- [x] `themux identify` shows context
- [x] CLI resolves socket path from env/flag/default

### 0.5 Python Test Suite

- [x] `conftest.py` socket fixture connects successfully
- [x] `test_v2_protocol.py` tests ready: ping, identify, capabilities, unknown method
- [x] Tests runnable via `cd tests && pytest -v`

## Exit Criteria

```
✓ libghostty-vt builds as shared library
✓ Rust FFI bindings wrap the C API
✓ system.ping responds via socket
✓ CLI ping/via socket works
✓ Python protocol tests ready
```

## Notes

Ghostty is used as **libghostty-vt** (core terminal emulation engine only), not the full Ghostty GTK app. The VT library provides terminal state management, escape sequence parsing, and input encoding. Our own GTK4 app wraps it via FFI bindings in `crates/ghostty-sys/`.

## Build Instructions

```bash
# One-time: Build libghostty-vt
./scripts/build-libghostty.sh

# Build Rust workspace
cargo build --workspace

# Run the app
cargo run -p themux-app
```

## Related

- [[development-roadmap]] — Full phase overview
- [[architecture]] — How the socket and terminal fit together
- [[building-checklist]] — Build verification
- [[phase-1-core-multiplexer]] — Next phase
