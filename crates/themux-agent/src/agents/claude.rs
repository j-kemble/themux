// Claude Code integration.
//
// Claude Code's teammate mode uses tmux under the hood. Our shim intercepts
// tmux commands and routes them through the themux socket.

/// Set up env vars for Claude Code to use themux as its tmux backend.
pub fn claude_env_vars() -> Vec<(&'static str, String)> {
    vec![
        ("TMUX", "/tmp/themux-claude/default".to_string()),
        ("TERM", "screen-256color".to_string()),
    ]
}
