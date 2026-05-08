// Hermes Agent integration.

pub fn hermes_env_vars() -> Vec<(&'static str, String)> {
    vec![
        ("TMUX", "/tmp/themux-hermes/default".to_string()),
        ("TERM", "screen-256color".to_string()),
    ]
}
