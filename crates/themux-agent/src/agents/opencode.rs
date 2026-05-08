// OpenCode integration ("oh-my-opencode" / omo).

pub fn opencode_env_vars() -> Vec<(&'static str, String)> {
    vec![
        ("TMUX", "/tmp/themux-opencode/default".to_string()),
        ("TERM", "screen-256color".to_string()),
    ]
}
