# Phase 4: Ecosystem

> **Status:** Not started | **Last updated:** 2026-05-08 | **Weeks:** 14+ | **Goal:** Cloud VMs, Flatpak, community readiness

## Prerequisites

- [ ] [[phase-3-session-polish]] complete

## Deliverables

### 4.1 Cloud VM Integration

- [ ] `themux vm list` — list user's cloud VMs
- [ ] `themux vm create` — create cloud VM (E2B, Freestyle providers)
- [ ] `themux vm shell` — attach terminal to cloud VM
- [ ] `themux vm destroy` — destroy cloud VM
- [ ] `themux vm ssh-info` — get SSH endpoint
- [ ] `themux vm exec` — execute command on VM
- [ ] Cloud VM workspace: `vm:<shortId>` naming
- [ ] Requires web platform (Next.js on Vercel) operational

### 4.2 Advanced Panels

- [ ] Markdown panel: render `.md` files
- [ ] File preview panel: quick look for images, PDFs, text
- [ ] File explorer sidebar: directory tree with git status
- [ ] Multiple file explorer visual styles

### 4.3 Task Manager

- [ ] Per-workspace CPU/memory/process monitoring
- [ ] `themux top` CLI command
- [ ] Task manager floating window (optional)

### 4.4 Feed / Workstream

- [ ] Agent workstream events: permission requests, exit plans
- [ ] Blocking hook semantics with timeout
- [ ] Feed panel in right sidebar
- [ ] Reply to agents from feed cards

### 4.5 Browser Enhancements

- [ ] Developer tools / inspector
- [ ] Browser history import from other browsers
- [ ] Per-WKWebView proxy observability
- [ ] Drag-and-drop files into browser

### 4.6 Packaging & Distribution

- [ ] Flatpak manifest and build
- [ ] Publish on Flathub
- [ ] AUR package
- [ ] `.deb` package
- [ ] `.rpm` package
- [ ] AppImage
- [ ] Auto-update mechanism (Flatpak handles this)

### 4.7 Documentation

- [ ] User documentation (docs site)
- [ ] API reference for V2 protocol
- [ ] Agent hook setup guides per agent
- [ ] Video demos
- [ ] Migration guide from cmux (macOS) to themux (Linux)

### 4.8 Community

- [ ] GitHub Discussions enabled
- [ ] Contributing guide polished
- [ ] Good first issues tagged
- [ ] Discord or Matrix community
- [ ] Release notes and changelog
- [ ] PostHog analytics (opt-in)

### 4.9 CI/CD

- [ ] GitHub Actions: build + test on push/PR
- [ ] GitHub Actions: Flatpak build
- [ ] GitHub Actions: publish release artifacts
- [ ] Automated version bumping
- [ ] Changelog generation

## Exit Criteria

```
✓ Flatpak available on Flathub
✓ Cloud VM creation and attachment works
✓ User docs published
✓ Community contribution workflow established
```

## Related

- [[phase-3-session-polish]] — Previous phase
- [[development-roadmap]] — Full overview
- [[project-goals]] — What we set out to build
- [[architecture]] — How these pieces fit
