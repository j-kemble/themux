# Phase 0: Foundation

> **Status:** Not started | **Last updated:** 2026-05-08 | **Weeks:** 1-2 | **Goal:** Shell app with terminal, basic socket server, CLI compatibility

## Prerequisites

- [ ] Rust toolchain installed (`rustc`, `cargo`)
- [ ] GTK4, WebKitGTK, libadwaita dev packages installed
- [ ] Zig installed (for Ghostty build)
- [ ] Go installed (for cmuxd-remote)
- [ ] `./scripts/setup.sh` runs successfully
- [ ] Ghostty submodule initialized and building
- [ ] `cargo build --workspace` passes

## Deliverables

### 0.1 Build System

- [ ] Cargo workspace compiles all 6 crates cleanly
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo test --workspace` passes (even if tests are empty)
- [ ] Makefile targets work: `build`, `test`, `clean`, `run`
- [ ] GitHub Actions CI passes on push

### 0.2 Ghostty Terminal Integration

- [ ] Ghostty submodule cloned and pinned to compatible commit
- [ ] Ghostty GTK widget renders in a `themux-app` window
- [ ] Terminal accepts keyboard input
- [ ] Terminal renders shell output
- [ ] Font rendering works (ligatures, CJK if applicable)
- [ ] Ghostty config (`~/.config/ghostty/config`) is loaded
- [ ] Terminal supports scrollback
- [ ] Terminal supports copy/paste
- [ ] Terminal resize works

### 0.3 Basic Socket Server

- [ ] Unix socket listener starts on `~/.local/share/themux/themux.sock`
- [ ] Socket auth (password mode) works
- [ ] `system.ping` returns `{"pong": true}`
- [ ] `system.identify` returns server info
- [ ] `system.capabilities` returns feature list
- [ ] Unknown methods return `method_not_found` error
- [ ] Socket cleanup on shutdown (remove stale socket file)

### 0.4 Basic CLI

- [ ] `themux ping` works via socket
- [ ] `themux version` prints version
- [ ] `themux capabilities` lists features
- [ ] `themux identify` shows context
- [ ] CLI resolves socket path from env/flag/default

### 0.5 Python Test Suite

- [ ] `conftest.py` socket fixture connects successfully
- [ ] `test_v2_protocol.py` passes: ping, identify, capabilities, unknown method
- [ ] Tests runnable via `cd tests && pytest -v`

## Exit Criteria

```
✓ Terminal renders in GTK4 window
✓ Keyboard input reaches the shell
✓ system.ping responds via socket
✓ CLI ping/via socket works
✓ Python protocol tests pass
```

## Related

- [[development-roadmap]] — Full phase overview
- [[architecture]] — How the socket and terminal fit together
- [[building-checklist]] — Build verification
- [[phase-1-core-multiplexer]] — Next phase
