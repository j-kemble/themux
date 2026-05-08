# Project Structure

> **Status:** Reference | **Last updated:** 2026-05-08 (updated after Phase 0)

## Top-Level Layout

```
themux/
├── cmuxaltprd.md              # Original Linux port PRD
├── Cargo.toml                 # Workspace root (7 crates)
├── Makefile                   # Build orchestration
├── README.md                  # Project overview
├── LICENSE                    # GPL-3.0-or-later
├── .gitignore
├── .gitmodules                # Ghostty submodule (legacy — ghostty now vendored)
│
├── crates/
│   ├── themux-core/           # Pure data models + business logic
│   │   ├── src/
│   │   │   ├── lib.rs         # Crate root, module declarations
│   │   │   ├── workspace/     # Workspace, Pane, Panel types + Manager
│   │   │   ├── config/        # cmux.json loader, schema, shortcuts
│   │   │   ├── layout/        # Split tree (Bonsplit replacement)
│   │   │   ├── session/       # Snapshot + persistence
│   │   │   ├── notification/  # Notification store
│   │   │   └── remote/        # SSH remote workspace config
│   │   └── Cargo.toml
│   │
│   ├── themux-socket/         # Unix socket server + V2 protocol
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── server.rs      # Unix listener, auth, V2 dispatch
│   │   │   ├── protocol/      # V2 dispatch, auth, streaming
│   │   │   └── event_bus.rs   # Ring buffer event system
│   │   └── Cargo.toml
│   │
│   ├── themux-cli/            # CLI binary (the `themux` command)
│   │   ├── src/
│   │   │   ├── main.rs        # Clap CLI definition + socket dispatch
│   │   │   ├── commands/      # Per-domain command stubs
│   │   │   ├── shell.rs       # POSIX shell quoting
│   │   │   └── tmux_compat.rs # Tmux command translation layer
│   │   └── Cargo.toml
│   │
│   ├── themux-agent/          # AI agent hook system
│   │   ├── src/
│   │   │   ├── lib.rs         # AgentKind enum
│   │   │   ├── hooks.rs       # Tmux shim installer
│   │   │   ├── agents/        # Per-agent env vars (Claude, Codex, etc.)
│   │   │   └── vault.rs       # Custom agent registration
│   │   └── Cargo.toml
│   │
│   ├── themux-notify/         # Desktop notification + OSC parser
│   │   ├── src/
│   │   │   ├── lib.rs         # Desktop notification sender
│   │   │   ├── osc.rs         # OSC 9/99/777 sequence parser
│   │   │   └── notification.rs
│   │   └── Cargo.toml
│   │
│   ├── ghostty-sys/           # Rust FFI bindings to libghostty-vt
│   │   ├── src/
│   │   │   └── lib.rs         # Re-exports bindgen-generated bindings
│   │   ├── build.rs            # bindgen + header path config
│   │   └── Cargo.toml
│   │
│   └── themux-app/            # GTK4 application binary
│       ├── src/
│       │   ├── main.rs        # Entry point, GTK app init
│       │   ├── app.rs         # Window builder, sidebar, content
│       │   ├── ui/            # Sidebar, content, terminal, browser, panels
│       │   │   ├── terminal_view.rs  # GTK widget holding Ghostty terminal
│       │   │   ├── terminal.rs       # TerminalWidget — libghostty-vt wrapper
│       │   │   ├── mod.rs
│       │   │   └── ...
│       │   └── ...
│       ├── build.rs            # Links libghostty-vt + sets include path
│       └── Cargo.toml
│
├── ghostty/                   # Vendored Ghostty source (libghostty-vt only)
│   ├── build.zig              # Zig build system
│   ├── build.zig.zon          # Zig dependency manifest
│   └── src/                   # Source (terminal emulation core only)
│
├── build/                     # Build artifacts
│   └── libghostty/            # libghostty-vt.so + headers
│       ├── lib/
│       └── include/
│
├── ui/                        # GTK4 resource files
│   ├── themux.css             # Application stylesheet
│   └── themux.gresource.xml   # GResource manifest
│
├── daemon/                    # cmuxd-remote Go daemon slot (not yet populated)
│
├── data/
│   └── default-config.json    # Bundled default config
│
├── docs/
│   ├── architecture.md        # System architecture document
│   ├── protocol-v2.md         # V2 JSON-RPC spec
│   └── contributing.md        # Contributor guide
│
├── scripts/
│   ├── setup.sh               # Full project setup (legacy)
│   ├── build.sh               # Build with mode selection
│   ├── build-libghostty.sh    # Build libghostty-vt .so + headers
│   ├── run.sh                 # Build + launch
│   └── run-tests.sh           # Rust + Python tests
│
├── tests/                     # Python integration tests
│   ├── conftest.py            # Socket connection fixture
│   ├── requirements.txt       # pytest
│   ├── test_v2_protocol.py    # System ping, identify, capabilities
│   ├── test_workspace_crud.py # Workspace create/list
│   ├── test_browser_api.py    # Browser automation protocol
│   └── test_tmux_compat.py    # Tmux compat protocol tests
│
├── themux-vault/              # Obsidian knowledge base (this vault)
└── .github/workflows/ci.yml   # CI pipeline
```

## Crate Descriptions

| Crate | Responsibility | Deps |
|-------|---------------|------|
| `themux-core` | Domain models, config, session, layout | serde, uuid, chrono, rusqlite, tokio |
| `themux-socket` | Socket server, V2 dispatch, event bus | core, tokio, hmac, sha2, rand |
| `themux-cli` | CLI binary, ~80 commands, tmux compat | core, socket, agent, clap |
| `themux-agent` | Agent hooks, shim installer, vault | core |
| `themux-notify` | Desktop notifications, OSC parser | core, notify-rust |
| `ghostty-sys` | libghostty-vt FFI bindings | bindgen (build) |
| `themux-app` | GTK4 GUI binary + Ghostty terminal | core, socket, notify, ghostty-sys, gtk4, webkit2gtk |

## Current Setup Status

As of 2026-05-08 (end of Phase 0):

- Git repository initialized at `github.com/j-kemble/themux` (master branch)
- Ghostty source vendored at `ghostty/` — trimmed to libghostty-vt only (terminal emulation core, no fonts/GTK/renderer)
- `scripts/build-libghostty.sh` builds `libghostty-vt.so` + headers into `build/libghostty/`
- `crates/ghostty-sys/` provides Rust FFI bindings via bindgen
- CLI (`themux-cli`) has socket dispatch wired for ping/version/capabilities/identify
- Socket server (`themux-socket`) has proper V2 JSON-RPC dispatch for system.* methods
- `cargo check --workspace` passes with 0 errors
- `daemon/cmuxd-remote/` is not yet present; daemon build targets will fail until populated

## Dependency Rules

1. **`themux-core` MUST NOT import any GTK, tokio, or platform crate.** Pure data only.
2. **Crates depend downward only**: `app` → `socket` → `core`, never `core` → `socket`.
3. **`themux-socket`** can use tokio (async socket I/O).
4. **`themux-app`** can use GTK4, WebKitGTK, tokio.
5. **`themux-notify`** can use notify-rust (platform notification bridge).

## Related

- [[architecture]] — How these pieces connect at runtime
- [[design-decisions]] — Why this structure
- [[build-principles]] — Rules for working within this structure
- [[building-checklist]] — Step-by-step build guide
