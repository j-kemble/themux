// Codex integration.

pub fn codex_env_vars() -> Vec<(&'static str, String)> {
    vec![
        ("TMUX", "/tmp/themux-codex/default".to_string()),
        ("TERM", "screen-256color".to_string()),
    ]
}
