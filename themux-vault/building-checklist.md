# Building Checklist

> **Status:** Active | **Last updated:** 2026-05-08
> Step-by-step verification for building and testing themux.

## One-Time Setup

```bash
# 1. Clone
git clone <repo-url> themux
cd themux

# 2. Install system dependencies (Fedora)
sudo dnf install rust cargo zig go python3-devel pkg-config \
    gtk4-devel webkit2gtk4.1-devel libadwaita-devel

# 3. Build libghostty-vt (the core terminal emulation library)
./scripts/build-libghostty.sh

# 4. Build Rust workspace
cargo build --workspace
```

- [x] This directory is a git repository (`test -d .git` succeeds)
- [x] `rustc --version` shows Rust 1.85+
- [x] `cargo --version` succeeds
- [x] `zig version` shows Zig 0.15+
- [x] `go version` shows Go 1.22+
- [x] `pkg-config --modversion gtk4` succeeds
- [x] `pkg-config --modversion webkit2gtk-4.1` succeeds
- [x] `ghostty/build.zig` exists (vendored source)
- [ ] `daemon/cmuxd-remote/` exists before building the daemon target
- [x] `./scripts/build-libghostty.sh` completes without errors

### Current local setup caveats

Obsolete — all prerequisites are now satisfied. See [[project-structure#Current Setup Status]] for current state.

## Daily Build

```bash
# Build libghostty-vt first (only needed if ghostty source changes)
./scripts/build-libghostty.sh

# Debug build (fast compile)
cargo build

# Release build (optimized)
cargo build --release
```

- [ ] `./scripts/build-libghostty.sh` passes — libghostty-vt builds
- [x] `cargo build --workspace` passes — all 7 crates compile
- [ ] `cargo build --release` passes — optimized build works

## Before Commit

```bash
# Format
cargo fmt --all
git diff --exit-code  # Should show no changes

# Lint
cargo clippy --workspace -- -D warnings

# Test
cargo test --workspace
```

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes (zero warnings)
- [ ] `cargo test --workspace` passes (all tests green)

## Protocol Tests

```bash
# Start themux first (in another terminal)
./scripts/run.sh

# Run Python tests
cd tests
pip install -r requirements.txt
pytest -v
```

- [ ] `test_v2_protocol.py` — all pass
- [ ] `test_workspace_crud.py` — all pass (if Phase 1+)
- [ ] `test_browser_api.py` — all pass (if Phase 2+)
- [ ] `test_tmux_compat.py` — all pass (if Phase 2+)

## Run the App

```bash
# Build libghostty-vt first
./scripts/build-libghostty.sh

# Debug mode
cargo run -p themux-app

# With logging
RUST_LOG=themux=debug cargo run -p themux-app

# Release
cargo run --release -p themux-app
```

- [ ] GTK4 window opens
- [ ] Sidebar visible on left
- [ ] Terminal widget renders (libghostty-vt placeholder)
- [ ] Keyboard input reaches terminal
- [ ] Resize works
- [ ] Close window exits cleanly

## Smoketest Sequence

After build, verify:

1. **Launch:** Window opens, terminal renders
2. **New workspace:** `themux new-workspace --name test` in another terminal
3. **Split:** `themux new-pane --direction horizontal` creates split
4. **Send text:** `themux send "echo hello"` sends to terminal
5. **Notify:** `themux notify --title "Test" --body "Hello"` fires notification
6. **List:** `themux list-workspaces` shows test workspace
7. **Close:** `themux close-workspace --workspace <id>` closes it
8. **Quit:** Close window, verify clean shutdown, no stale socket

## CI Pipeline

GitHub Actions runs on every push and PR:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace -- -D warnings`
3. `cargo build --workspace`
4. `cargo test --workspace`

Check CI status at: `.github/workflows/ci.yml`

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `gtk4-sys` not found | `sudo dnf install gtk4-devel` |
| `webkit2gtk` not found | `sudo dnf install webkit2gtk4.1-devel` |
| Stale socket file | `rm ~/.local/share/themux/themux.sock` |
| `cargo build` slow first time | Normal — building all deps. Subsequent builds use cache. |
| `libghostty-vt` build fails | Check Zig version (needs >=0.15.2): `zig version`. If 0.16, install 0.15.2 from ziglang.org. |
| `ghostty-sys` bindgen fails | Run `./scripts/build-libghostty.sh` first, then `cargo clean && cargo build` |
| OSC tests fail | Check terminal type: must be `xterm-256color` or similar |

## Related

- [[build-principles]] — Rules that motivate this checklist
- [[agent-instructions]] — How agents follow this checklist
- [[development-roadmap]] — Phase-specific verification
- [[phase-0-foundation]] — Phase 0 exit criteria
