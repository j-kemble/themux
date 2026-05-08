# Phase 2: Browser + Agents

> **Status:** Not started | **Last updated:** 2026-05-08 | **Weeks:** 7-10 | **Goal:** In-app browser, agent hooks, tmux compat, SSH workspaces

## Prerequisites

- [ ] [[phase-1-core-multiplexer]] complete

## Deliverables

### 2.1 In-App Browser (WebKitGTK)

- [ ] WebKitGTK WebView as panel type in `themux-app`
- [ ] Browser panels coexist with terminal panels in splits
- [ ] Address bar with search engine support (Google, DuckDuckGo, Bing)
- [ ] `browser.navigate` — navigate to URL
- [ ] `browser.eval` / `browser.eval_async` — execute JavaScript
- [ ] `browser.screenshot` — capture page image
- [ ] `browser.element` — query DOM elements
- [ ] `browser.click`, `browser.type`, `browser.scroll` — interact
- [ ] `browser.back`, `browser.forward`, `browser.reload`
- [ ] `browser.url.get` — get current URL
- [ ] `browser.dialog.accept`, `browser.dialog.dismiss` — JS dialogs
- [ ] `browser.cookies.get`, `browser.cookies.set`, `browser.cookies.clear`
- [ ] `browser.storage.get`, `browser.storage.set`, `browser.storage.clear`
- [ ] `browser.tab.new`, `browser.tab.list`, `browser.tab.switch`, `browser.tab.close`
- [ ] `browser.console.list`, `browser.console.clear` — JS console
- [ ] `browser.errors.list` — page errors
- [ ] `browser.highlight` — highlight elements
- [ ] `browser.addinitscript`, `browser.addscript`, `browser.addstyle` — injection
- [ ] `browser.open_split` — open URL in new browser split
- [ ] `browser.focus_webview` — focus browser panel
- [ ] Multiple browser profiles (isolated storage)

### 2.2 Agent Hooks

- [ ] Tmux shim installer in `themux-agent`
- [ ] Shims: `~/.themux/agent-bin/<agent>/tmux`
- [ ] Shims: `~/.themux/agent-bin/<agent>/terminal-notifier`
- [ ] Agent env vars: `TMUX`, `TMUX_PANE`, `TERM=screen-256color`
- [ ] Agent env vars: `CMUX_SOCKET_PATH`, `CMUX_WORKSPACE_ID`, `CMUX_SURFACE_ID`
- [ ] **Claude Code / Claude Teams:** `themux claude-teams` launches with teammate mode
- [ ] **Codex:** `themux omx` launches with tmux shim
- [ ] **OpenCode:** `themux omo` launches with oh-my-opencode plugin
- [ ] **Hermes Agent:** `themux hooks hermes-agent install`
- [ ] `themux hooks install <agent>` for all supported agents
- [ ] `themux hooks uninstall <agent>`
- [ ] `themux hooks list`

### 2.3 Tmux Compatibility

- [ ] `themux __tmux-compat` command (hidden)
- [ ] `new-window` → `workspace.create`
- [ ] `split-window` → `surface.split`
- [ ] `send-keys` → `surface.send_text`
- [ ] `capture-pane` → `surface.read_text`
- [ ] `select-pane` → `pane.focus`
- [ ] `kill-pane` → `surface.close`
- [ ] `list-panes` → `pane.list`
- [ ] `resize-pane` → `pane.resize`
- [ ] `display-message` → log passthrough
- [ ] `last-window`, `next-window`, `previous-window`
- [ ] Fake `TMUX` socket path for agent detection

### 2.4 SSH Remote Workspaces

- [ ] cmuxd-remote Go daemon builds on Linux
- [ ] Daemon binary bundled with themux
- [ ] `themux ssh --destination user@host` creates remote workspace
- [ ] Remote daemon bootstrap: upload binary, start over SSH
- [ ] WebSocket PTY bridge: themux terminal ↔ cmuxd-remote ↔ remote shell
- [ ] Browser proxy through SSH tunnel
- [ ] Remote port forwarding
- [ ] Reconnection with exponential backoff
- [ ] `themux ssh-session-end` disconnects
- [ ] `workspace.list` shows transport state

### 2.5 Agent Session Resume

- [ ] `RestorableAgentSession` in `themux-core`
- [ ] Detect agent processes (Claude Code PID, Codex PID)
- [ ] Persist agent session IDs in workspace snapshot
- [ ] On session restore: send resume command to terminal
- [ ] Supported agents: Claude Code, Codex, OpenCode, Hermes, RovoDev

### 2.6 Protocol Tests

- [ ] `test_browser_api.py` covers all browser methods
- [ ] Agent hook tests: tmux compat translation
- [ ] SSH workspace create/connect/disconnect

## Exit Criteria

```
✓ Browser panel renders alongside terminal in split pane
✓ browser.navigate, .eval, .screenshot work via socket
✓ Claude Code launches in split pane with teammate mode
✓ tmux new-window creates themux workspace
✓ SSH workspace connects and shows remote shell
✓ Browser routes through SSH tunnel on remote workspaces
```

## Related

- [[phase-1-core-multiplexer]] — Previous phase
- [[phase-3-session-polish]] — Next phase
- [[development-roadmap]] — Full overview
- [[protocol-v2]] — Browser API methods
