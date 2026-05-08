# themux Development Knowledge Base

> **Status:** Active | **Last updated:** 2026-05-08
> Map of Content — start here for everything themux.

## Core Documents

- [[project-goals]] — Vision, scope, and non-goals
- [[architecture]] — System design, crate graph, data flow
- [[design-decisions]] — Key technical choices and rationale
- [[project-structure]] — Directory layout, current setup status, and file map

## Process & Standards

- [[build-principles]] — Conventions, standards, commit style
- [[dos-and-donts]] — Best practices, anti-patterns, pitfalls
- [[building-checklist]] — Setup, build, verify steps, and current setup caveats
- [[agent-instructions]] — How AI agents should work on this project

## Planning & Roadmap

- [[development-roadmap]] — Master phase plan with timelines
- [[phase-0-foundation]] — Shell app, terminal, basic socket
- [[phase-1-core-multiplexer]] — Workspaces, splits, sidebar, notifications
- [[phase-2-browser-agents]] — Browser, agent hooks, SSH
- [[phase-3-session-polish]] — Persistence, settings, quality
- [[phase-4-ecosystem]] — Cloud VMs, advanced features

## Technical Reference

- [[protocol-v2]] — V2 JSON-RPC target contract plus current implementation status
- [[review-2026-05-08]] — Vault/project documentation review and current setup gaps
- [Original PRD](../cmuxaltprd.md) — Linux port PRD at repo root
- [Repo architecture doc](../docs/architecture.md) — public-facing architecture summary
- [Repo protocol doc](../docs/protocol-v2.md) — public-facing protocol summary
- [Contributing guide](../docs/contributing.md) — contributor setup and conventions

## Quick Links

- **Repo:** `/home/josh/Projects/themux/`
- **Vault:** `/home/josh/Projects/themux/themux-vault/`
- **PRD:** `/home/josh/Projects/themux/cmuxaltprd.md`
- **Build:** `cd /home/josh/Projects/themux && cargo build --workspace` or `./scripts/build.sh`
- **Run:** `./scripts/run.sh`
- **Test:** `./scripts/run-tests.sh` or `cargo test --workspace`

## Related

- [[project-goals]] — Why this project exists
- [[project-structure]] — What is in the repository today
- [[building-checklist]] — How to verify the setup
- [[agent-instructions]] — Start here before coding with an agent

---

*Built by Hermes Agent (Pip) — 2026-05-08*
