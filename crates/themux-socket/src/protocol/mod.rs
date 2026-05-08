// V2 JSON-RPC protocol handler.
//
// Implements the method dispatch table for cmux's V2 protocol.
// Method namespaces:
//   system.*    - ping, identify, capabilities
//   workspace.* - list, current, create, select, close, rename, reorder, action
//   pane.*      - list, surfaces, create, focus
//   surface.*   - list, focus, close, create, split, split_off, move, reorder,
//                 rename, send_text, send_key, read_text, action, health, trigger_flash
//   browser.*   - navigate, eval, eval_async, back, forward, reload, url.get,
//                 screenshot, element, click, type, scroll, dialog.*, frame.*,
//                 cookies.*, storage.*, tab.*, state.*, console.*, errors.*,
//                 highlight, addinitscript, addscript, addstyle, focus_webview, open_split
//   notification.* - create, create_for_caller
//   auth.*      - status, begin_sign_in, sign_out
//   settings.*  - open
//   events.*    - stream
//   vm.*        - list, create, destroy, ssh_info, exec, ssh_attach, pty_attach
//   markdown.*  - open
//   debug.*     - terminals

pub mod auth;
pub mod stream;
pub mod v2;
