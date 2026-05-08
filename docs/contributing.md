# Contributing to themux

## Development Setup

1. Install prerequisites (Fedora):
```bash
sudo dnf install rust cargo zig go python3-devel pkgconfig \
    gtk4-devel webkit2gtk4.1-devel libadwaita-devel
```

2. Clone and setup:
```bash
git clone --recurse-submodules https://github.com/themux-app/themux.git
cd themux
./scripts/setup.sh
```

3. Build:
```bash
cargo build
```

4. Run:
```bash
./scripts/run.sh
```

## Project Conventions

- **Rust edition:** 2021
- **Formatting:** `cargo fmt`
- **Linting:** `cargo clippy -- -D warnings`
- **Testing:** `cargo test --workspace`
- **Commits:** Conventional commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`)

## Code Organization

- `crates/themux-core/` — Pure data models, no I/O outside of persistence. No GTK dependencies.
- `crates/themux-socket/` — Socket server, protocol dispatch. Depends on core.
- `crates/themux-cli/` — CLI binary. Depends on core, socket, agent.
- `crates/themux-agent/` — Agent hook management. Depends on core.
- `crates/themux-notify/` — Desktop notifications. Depends on core.
- `crates/themux-app/` — GTK4 binary. Depends on core, socket, notify.

## Adding a New V2 Method

1. Add the method variant to `crates/themux-socket/src/protocol/v2.rs`
2. Implement the handler in the dispatch match
3. Add the method to `docs/protocol-v2.md`
4. Add a test in `tests/test_v2_protocol.py`

## Before Submitting

- [ ] `cargo fmt --all` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] New methods are documented in protocol-v2.md
- [ ] Commit messages follow conventional commits format
