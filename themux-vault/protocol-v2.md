# V2 JSON-RPC Protocol Reference

> **Status:** Reference | **Last updated:** 2026-05-08 | **Source:** [[architecture]]

## Transport

- **Mode:** Unix domain socket (`SOCK_STREAM`)
- **Default path:** `~/.local/share/themux/themux.sock`
- **Framing:** Newline-delimited JSON (one object per `\n`)
- **Encoding:** UTF-8

## Authentication

Password challenge (optional, configurable per socket mode):

```
Client → Server:  auth <password>\n
Server → Client:  OK\n           (success)
Server → Client:  ERROR: ...\n   (failure)
```

Password resolution order:
1. `--password` CLI flag
2. `THEMUX_SOCKET_PASSWORD` env var
3. `~/.local/share/themux/socket-password` file
4. Secret Service (TBD)

## Message Format

### Request
```json
{
  "id": "<uuid-v4>",
  "method": "<namespace.action>",
  "params": { ... }
}
```

### Success Response
```json
{
  "id": "<uuid-v4>",
  "ok": true,
  "result": { ... }
}
```

### Error Response
```json
{
  "id": "<uuid-v4>",
  "ok": false,
  "error": {
    "code": "<error_code>",
    "message": "<human description>"
  }
}
```

### Stream Mode
For `events.stream`: after initial response, server pushes JSON events until client disconnects.

### Heartbeat
Every 15 seconds of silence, server sends: `{"type":"heartbeat"}`
Client must handle this without disconnecting.

## Method Index

### system.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `system.ping` | `{}` | `{"pong": true}` | 0 |
| `system.identify` | `{}` | `{"server":"themux","version":"...","protocol":"v2"}` | 0 |
| `system.capabilities` | `{}` | `{"protocols":["v1","v2"],"features":[...]}` | 0 |

### workspace.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `workspace.list` | `{}` | `{"workspaces":[...]}` | 1 |
| `workspace.current` | `{}` | `{"workspace_id":"...","title":"..."}` | 1 |
| `workspace.create` | `{cwd?, name?, description?, command?, layout?, focus?}` | `{"workspace_id":"..."}` | 1 |
| `workspace.select` | `{workspace_id}` | `{"selected":true}` | 1 |
| `workspace.close` | `{workspace_id}` | `{"closed":true}` | 1 |
| `workspace.rename` | `{workspace_id, title}` | `{"renamed":true}` | 1 |
| `workspace.reorder` | `{workspace_id, index}` | `{"reordered":true}` | 1 |
| `workspace.action` | `{workspace_id, action}` | `{}` | 1 |
| `workspace.move_to_window` | `{workspace_id, window_id}` | `{}` | 3 |

### pane.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `pane.list` | `{workspace_id?}` | `{"panes":[...]}` | 1 |
| `pane.create` | `{workspace_id?, type?, direction?, url?, focus?}` | `{"pane_id":"..."}` | 1 |
| `pane.focus` | `{pane_id?, direction?}` | `{}` | 1 |
| `pane.surfaces` | `{pane_id}` | `{"surfaces":[...]}` | 1 |

### surface.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `surface.list` | `{workspace_id?}` | `{"surfaces":[...]}` | 1 |
| `surface.focus` | `{surface_id}` | `{}` | 1 |
| `surface.close` | `{surface_id}` | `{}` | 1 |
| `surface.create` | `{workspace_id?, type, pane_id?, url?, focus?}` | `{"surface_id":"..."}` | 1 |
| `surface.split` | `{workspace_id?, surface_id?, direction?}` | `{"pane_id":"..."}` | 1 |
| `surface.split_off` | `{surface_id, direction}` | `{"pane_id":"..."}` | 2 |
| `surface.move` | `{surface_id, workspace_id?, pane_id?, window_id?, before?, after?, index?, focus?}` | `{}` | 1 |
| `surface.reorder` | `{surface_id, before?, after?, index?}` | `{}` | 1 |
| `surface.rename` | `{surface_id, title}` | `{}` | 1 |
| `surface.send_text` | `{surface_id?, text}` | `{"sent":true}` | 1 |
| `surface.send_key` | `{surface_id?, key}` | `{"sent":true}` | 1 |
| `surface.read_text` | `{surface_id?, lines?, scrollback?}` | `{"text":"..."}` | 1 |
| `surface.action` | `{surface_id, action}` | `{}` | 1 |
| `surface.health` | `{surface_id?}` | `{"healthy":true}` | 1 |
| `surface.trigger_flash` | `{surface_id?}` | `{}` | 1 |

### browser.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `browser.navigate` | `{surface_id?, url}` | `{"url":"..."}` | 2 |
| `browser.eval` | `{surface_id?, code}` | `{"result":...}` | 2 |
| `browser.eval_async` | `{surface_id?, code}` | `{"result":...}` | 2 |
| `browser.back` | `{surface_id?}` | `{}` | 2 |
| `browser.forward` | `{surface_id?}` | `{}` | 2 |
| `browser.reload` | `{surface_id?}` | `{}` | 2 |
| `browser.url.get` | `{surface_id?}` | `{"url":"..."}` | 2 |
| `browser.screenshot` | `{surface_id?}` | `{"data":"base64..."}` | 2 |
| `browser.element` | `{surface_id?, selector}` | `{"refs":[...],"count":N}` | 2 |
| `browser.click` | `{surface_id?, ref}` | `{}` | 2 |
| `browser.type` | `{surface_id?, ref, text}` | `{}` | 2 |
| `browser.scroll` | `{surface_id?, direction?, amount?}` | `{}` | 2 |
| `browser.dialog.accept` | `{surface_id?, text?}` | `{}` | 2 |
| `browser.dialog.dismiss` | `{surface_id?}` | `{}` | 2 |
| `browser.cookies.get` | `{surface_id?, url?}` | `{"cookies":[...]}` | 2 |
| `browser.cookies.set` | `{surface_id?, name, value, url?}` | `{}` | 2 |
| `browser.cookies.clear` | `{surface_id?, url?}` | `{}` | 2 |
| `browser.console.list` | `{surface_id?}` | `{"messages":[...]}` | 2 |
| `browser.console.clear` | `{surface_id?}` | `{}` | 2 |
| `browser.errors.list` | `{surface_id?}` | `{"errors":[...]}` | 2 |
| `browser.highlight` | `{surface_id?, ref}` | `{}` | 2 |
| `browser.addinitscript` | `{surface_id?, code}` | `{}` | 2 |
| `browser.addscript` | `{surface_id?, code}` | `{}` | 2 |
| `browser.addstyle` | `{surface_id?, css}` | `{}` | 2 |
| `browser.focus_webview` | `{surface_id?}` | `{}` | 2 |
| `browser.open_split` | `{workspace_id?, url}` | `{"surface_id":"..."}` | 2 |
| `browser.tab.new` | `{surface_id?}` | `{"tab_id":"..."}` | 2 |
| `browser.tab.list` | `{surface_id?}` | `{"tabs":[...]}` | 2 |
| `browser.tab.switch` | `{surface_id?, tab_id}` | `{}` | 2 |
| `browser.tab.close` | `{surface_id?, tab_id}` | `{}` | 2 |
| `browser.frame.select` | `{surface_id?, selector}` | `{}` | 2 |
| `browser.frame.main` | `{surface_id?}` | `{}` | 2 |
| `browser.state.save` | `{surface_id?}` | `{"state":"..."}` | 3 |
| `browser.state.load` | `{surface_id?, state}` | `{}` | 3 |
| `browser.storage.get` | `{surface_id?, key, storage?}` | `{"value":"..."}` | 2 |
| `browser.storage.set` | `{surface_id?, key, value, storage?}` | `{}` | 2 |
| `browser.storage.clear` | `{surface_id?, storage?}` | `{}` | 2 |

### notification.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `notification.create` | `{title, subtitle?, body?, category?}` | `{"notification_id":"..."}` | 1 |
| `notification.create_for_caller` | `{title, subtitle?, body?, category?}` | `{"notification_id":"..."}` | 1 |

### auth.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `auth.status` | `{}` | `{"signed_in":false}` | 3 |
| `auth.begin_sign_in` | `{}` | `{"url":"..."}` | 3 |
| `auth.sign_out` | `{}` | `{}` | 3 |

### settings.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `settings.open` | `{target?}` | `{}` | 3 |

### events.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `events.stream` | `{after_seq?, name?, category?, reconnect?, limit?}` | Stream | 1 |

### vm.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `vm.list` | `{}` | `{"vms":[...]}` | 4 |
| `vm.create` | `{image?, provider?}` | `{"vm_id":"..."}` | 4 |
| `vm.destroy` | `{vm_id}` | `{}` | 4 |
| `vm.ssh_info` | `{vm_id}` | `{"host":"...","port":22}` | 4 |
| `vm.exec` | `{vm_id, command}` | `{"output":"...","exit_code":0}` | 4 |
| `vm.ssh_attach` | `{vm_id}` | `{"workspace_id":"..."}` | 4 |

### markdown.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `markdown.open` | `{workspace_id?, path}` | `{"surface_id":"..."}` | 4 |

### debug.*

| Method | Params | Returns | Phase |
|--------|--------|---------|-------|
| `debug.terminals` | `{}` | `{"terminals":[...]}` | 0 |

## Current Implementation Status

This note documents the target V2 contract. The current scaffold implements only a small subset:

- `crates/themux-socket/src/protocol/v2.rs` currently dispatches `system.ping`, `system.identify`, `system.capabilities`, `workspace.list`, `workspace.create`, `surface.create`, and `surface.send_text`.
- `workspace.create` and `surface.create` currently return stub IDs.
- `crates/themux-socket/src/server.rs` does not yet call the full V2 dispatcher for incoming socket lines.
- `crates/themux-cli/src/main.rs` parses the CLI but still needs socket connection and command dispatch wiring.
- Browser, auth, settings, events, VM, markdown, and most workspace/pane/surface methods remain planned by phase.

Update this section whenever implementation catches up with the target method index.

## Error Codes

| Code | Meaning |
|------|---------|
| `method_not_found` | Method doesn't exist |
| `invalid_params` | Params failed validation |
| `workspace_not_found` | Referenced workspace ID doesn't exist |
| `pane_not_found` | Referenced pane ID doesn't exist |
| `surface_not_found` | Referenced surface ID doesn't exist |
| `not_implemented` | Method exists but isn't implemented yet |
| `not_supported` | Method can't work on this platform |
| `access_denied` | Authentication required or insufficient permissions |
| `no_browser_panel` | Browser operation requested but no browser panel exists |
| `internal_error` | Unexpected server error |

## Related

- [[architecture]] — How the protocol fits in the system
- [[phase-0-foundation]] — Phase 0 implements system.*
- [[phase-1-core-multiplexer]] — Phase 1 implements workspace.*, pane.*, surface.*, notification.*
- [[phase-2-browser-agents]] — Phase 2 implements browser.*
- [[project-structure]] — Current scaffold/setup status
