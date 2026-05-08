# Build Principles

> **Status:** Active | **Last updated:** 2026-05-08
> Rules, conventions, and standards for themux development.

## Language & Style

### Rust

- **Edition:** 2021
- **Format:** `cargo fmt` (always before commit)
- **Lint:** `cargo clippy --workspace -- -D warnings` (must pass)
- **Unsafe:** `#![warn(unsafe_code)]` — no unsafe without explicit justification
- **Docs:** `#![warn(missing_docs)]` on public API
- **Error handling:** `thiserror` for libraries, `anyhow` for binaries
- **Async:** `tokio` for socket server and I/O; GTK main thread for UI

### Commit Style

```
type(scope): description

- feat: new feature
- fix: bug fix
- docs: documentation
- refactor: code restructuring
- test: adding/updating tests
- chore: build, CI, tooling
```

### Naming

- **Workspace** = a container with a split tree of panes (was "Tab" in legacy cmux)
- **Pane** = a leaf in the split tree, holds panels
- **Panel** = a terminal, browser, markdown, or file preview surface
- **Surface** = alias for Panel (protocol-level naming)
- **Crate names:** `themux-<domain>` (lowercase, hyphens)

## Architecture Rules

1. **Protocol-first:** V2 JSON-RPC is the source of truth. UI and CLI are clients.
2. **Core is pure:** `themux-core` has zero platform deps. Testable headlessly.
3. **Downward deps only:** `app` → `socket` → `core`. Never `core` → `socket`.
4. **One responsibility per crate:** If a crate does two unrelated things, split it.
5. **Submodules over vendoring:** Ghostty and cmuxd-remote are git submodules.

## Testing

- **Unit tests:** `#[cfg(test)]` in the same file as the code
- **Integration tests:** In `crates/*/tests/` directory
- **Protocol tests:** Python test suite in `tests/` using V2 socket
- **TDD for new features:** Write the V2 protocol test first, then implement
- **Coverage:** Every V2 method must have a protocol test

## Building

- **Setup:** `./scripts/setup.sh` (once)
- **Build:** `cargo build` (debug), `cargo build --release` (release)
- **Run:** `cargo run -p themux-app` or `./scripts/run.sh`
- **Test:** `cargo test --workspace` + `cd tests && pytest -v`
- **CI:** GitHub Actions on every push/PR

## Documentation

- **Vault-first:** All dev docs live in [[themux-dev|this Obsidian vault]]
- **Inline docs:** `///` doc comments on all public items
- **Architecture:** [[architecture]] — keep it updated
- **Protocol:** [[protocol-v2]] — update when methods change
- **Decisions:** [[design-decisions]] — record rationale, not just outcomes

## When Adding Features

1. Read [[project-goals]] — is this in scope?
2. Read [[design-decisions]] — does it align?
3. Read [[development-roadmap]] — which phase?
4. Check [[architecture]] — where does it fit?
5. Add V2 protocol method to [[protocol-v2]]
6. Implement in correct crate (see [[project-structure]] for crate responsibilities)
7. Write protocol test in `tests/`
8. Update the phase checklist
9. Commit with conventional commit format

## Related

- [[dos-and-donts]] — Specific patterns to embrace or avoid
- [[design-decisions]] — Why these rules exist
- [[building-checklist]] — Step-by-step verification
- [[agent-instructions]] — How AI agents follow these rules
