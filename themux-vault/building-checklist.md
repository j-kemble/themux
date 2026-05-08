# Building Checklist

> **Status:** Active | **Last updated:** 2026-05-08
> Step-by-step verification for building and testing themux.

## One-Time Setup

```bash
# 1. Clone with submodules
git clone --recurse-submodules <repo-url> themux
cd themux

# 2. Install system dependencies (Fedora)
sudo dnf install rust cargo zig go python3-devel pkg-config \
    gtk4-devel webkit2gtk4.1-devel libadwaita-devel

# 3. Initialize/copy external components if this is the local scaffold
#    - ghostty/ must contain the Ghostty submodule checkout with build.zig
#    - daemon/cmuxd-remote/ must contain the cmux Go remote daemon

git submodule update --init --recursive
# Copy or symlink cmux daemon source if needed:
# cp -r <cmux-repo>/daemon/remote daemon/cmuxd-remote

# 4. Run setup (builds Ghostty, cmuxd-remote, Rust workspace)
./scripts/setup.sh
```

- [ ] This directory is a git repository (`test -d .git` succeeds)
- [ ] `rustc --version` shows Rust 1.85+
- [ ] `cargo --version` succeeds
- [ ] `zig version` shows Zig 0.14+
- [ ] `go version` shows Go 1.22+
- [ ] `pkg-config --modversion gtk4` succeeds
- [ ] `pkg-config --modversion webkit2gtk-4.1` succeeds
- [ ] `ghostty/build.zig` exists after submodule initialization
- [ ] `daemon/cmuxd-remote/` exists before building the daemon target
- [ ] `./scripts/setup.sh` completes without errors

### Current local setup caveats

Observed on 2026-05-08 in `/home/josh/Projects/themux`:

- `rustc`, `cargo`, `zig`, and `go` are not currently available in the execution PATH, so build/test commands cannot be verified from this shell yet.
- `pkg-config` is installed, but `gtk4`, `webkit2gtk-4.1` / `webkitgtk-6.0`, and `libadwaita-1` development packages are not visible to it yet.
- `.git/` is missing, so `git status` and submodule status checks fail until the directory is initialized/cloned as a repository.
- `ghostty/` exists but is empty; `ghostty/build.zig` is missing.
- `daemon/cmuxd-remote/` is missing; `scripts/setup.sh` will warn and continue, and `scripts/build.sh daemon`/`make build-daemon` will fail until it is copied in.

## Daily Build

```bash
# Debug build (fast compile)
cargo build

# Release build (optimized)
cargo build --release
```

- [ ] `cargo build --workspace` passes — all 6 crates compile
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
# Debug mode
cargo run -p themux-app

# With logging
RUST_LOG=themux=debug cargo run -p themux-app

# Release
cargo run --release -p themux-app
```

- [ ] GTK4 window opens
- [ ] Sidebar visible on left
- [ ] Terminal renders (Ghostty widget)
- [ ] Keyboard input reaches shell
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
| Ghostty GTK widget fails | Check Ghostty submodule: `git submodule update --init` |
| OSC tests fail | Check terminal type: must be `xterm-256color` or similar |

## Related

- [[build-principles]] — Rules that motivate this checklist
- [[agent-instructions]] — How agents follow this checklist
- [[development-roadmap]] — Phase-specific verification
- [[phase-0-foundation]] — Phase 0 exit criteria
