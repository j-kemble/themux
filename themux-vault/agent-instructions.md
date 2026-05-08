# Agent Instructions

> **Status:** Active | **Last updated:** 2026-05-08
> How AI coding agents (Claude Code, Codex, Hermes Agent, etc.) should work on the themux project.

## Project Context

themux is a **native Linux terminal multiplexer for AI coding agents** built in Rust with GTK4. It ports macOS cmux functionality to Linux. Read the MOC first: [[themux-dev]].

## Before Writing Code

1. **Read** [[project-goals]] — understand scope and non-goals
2. **Read** [[architecture]] — understand where code lives
3. **Read** [[design-decisions]] — understand the rationale
4. **Read** [[build-principles]] — understand the rules
5. **Read** [[dos-and-donts]] — avoid common mistakes
6. **Check** [[development-roadmap]] — which phase are we in?
7. **Check** the relevant [[phase-0-foundation|phase checklist]] for current tasks

## Crate Responsibilities

| Crate | What goes there |
|-------|----------------|
| `themux-core` | Domain types (`Workspace`, `Pane`, `Panel`), config loading, session snapshot, split tree, notification store. **No GTK. No platform deps.** |
| `themux-socket` | Unix socket listener, V2 JSON-RPC dispatch, auth, event bus. Uses tokio. |
| `themux-cli` | CLI binary. Parses args with clap, connects to socket, dispatches commands. |
| `themux-agent` | Agent hook installer: creates tmux/terminal-notifier shims in `~/.themux/agent-bin/`. |
| `themux-notify` | Desktop notification sender via notify-rust, OSC sequence parser. |
| `ghostty-sys` | Rust FFI bindings to libghostty-vt (auto-generated via bindgen). Depends on `build/libghostty/` being built first. |
| `themux-app` | GTK4 binary. Window, sidebar, terminal view, browser view. Uses `ghostty-sys` for terminal emulation. |

**If you're not sure where code goes:** see [[project-structure]].

## Workflow for New Features

1. **V2 Protocol First:** Add the method to `themux-socket/src/protocol/v2.rs`
2. **Core Model:** Add any needed types to `themux-core`
3. **Protocol Test:** Write a Python test in `tests/`
4. **Implementation:** Wire it up in the correct crate
5. **CLI:** Expose it via `themux-cli` if it's a user-facing command
6. **Docs:** Update [[protocol-v2]], the relevant [[development-roadmap|phase checklist]], and any affected architecture docs

## Commit Convention

```
feat(socket): add workspace.reorder method
fix(core): handle empty workspace list on select
docs(vault): update phase-0 checklist
refactor(socket): extract auth into protocol module
test: add browser.navigate protocol test
chore: update Cargo.toml dependencies
```

## Before Submitting

- [ ] `./scripts/build-libghostty.sh` passes (if ghostty source changed)
- [ ] `cargo fmt --all` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] Python tests pass: `cd tests && pytest -v`
- [ ] New V2 methods documented in [[protocol-v2]]
- [ ] Relevant phase checklist updated
- [ ] No `unwrap()` in library code
- [ ] No GTK deps in `themux-core`

## Common Tasks

### Adding a new V2 method

1. Add match arm in `themux-socket/src/protocol/v2.rs`:
   ```rust
   "workspace.reorder" => handle_workspace_reorder(request.params).await,
   ```
2. Add test in `tests/test_v2_protocol.py`:
   ```python
   def test_workspace_reorder(themux):
       resp = themux.rpc("workspace.reorder", {"workspace_id": "...", "index": 0})
       assert resp["ok"] is True
   ```
3. Update [[protocol-v2#workspace.*]] with params and return type

### Adding a new CLI command

1. Add variant to `Commands` enum in `themux-cli/src/main.rs`
2. Add handler module in `themux-cli/src/commands/`
3. Wire up socket dispatch

### Adding a new agent

1. Add variant to `AgentKind` enum in `themux-agent/src/lib.rs`
2. Add env vars in `themux-agent/src/agents/<name>.rs`
3. Add shim dir name mapping in `themux-agent/src/hooks.rs`

## Environment

- **Vault:** `/home/josh/Projects/themux/themux-vault/`
- **Repo:** `/home/josh/Projects/themux/`
- **Socket:** `~/.local/share/themux/themux.sock`
- **Config:** `~/.config/themux/config.json`
- **Data:** `~/.local/share/themux/`

## Related

- [[themux-dev]] — Map of Content
- [[build-principles]] — Rules to follow
- [[dos-and-donts]] — Patterns and anti-patterns
- [[building-checklist]] — Build verification
