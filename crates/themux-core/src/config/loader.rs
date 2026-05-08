// Config loader: read and merge themux config files.
//
// Search path (in order, later files override earlier):
//   1. ~/.config/themux/config.json  (primary)
//   2. ./.themux.json                 (project-level)
//
// Falls back to ~/.config/cmux/cmux.json for cmux compatibility.

use super::ThemuxConfig;
use std::path::PathBuf;

/// Load config from the standard search paths.
pub fn load() -> Result<ThemuxConfig, LoadError> {
    let paths = config_paths();

    // Start with defaults
    let mut config = ThemuxConfig::default();

    // Merge each file (later files override)
    for path in &paths {
        if path.exists() {
            let content = std::fs::read_to_string(path)
                .map_err(|e| LoadError::Io(path.clone(), e))?;
            let partial: ThemuxConfig = serde_json::from_str(&content)
                .map_err(|e| LoadError::Parse(path.clone(), e))?;
            // Simple field-level merge: overwrite non-empty fields
            merge_config(&mut config, partial);
        }
    }

    Ok(config)
}

fn config_paths() -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let mut paths = vec![
        home.join(".config/themux/config.json"),
        PathBuf::from(".themux.json"),
    ];
    // cmux compatibility fallback
    let cmux_path = home.join(".config/cmux/cmux.json");
    if cmux_path.exists() {
        paths.insert(0, cmux_path);
    }
    paths
}

fn merge_config(base: &mut ThemuxConfig, partial: ThemuxConfig) {
    if !partial.actions.is_empty() {
        base.actions = partial.actions;
    }
    if partial.ui.is_some() {
        base.ui = partial.ui;
    }
    if partial.new_workspace_command.is_some() {
        base.new_workspace_command = partial.new_workspace_command;
    }
    if !partial.commands.is_empty() {
        base.commands = partial.commands;
    }
    if !partial.vault.is_empty() {
        base.vault = partial.vault;
    }
    for (k, v) in partial.shortcuts {
        base.shortcuts.insert(k, v);
    }
    if partial.appearance.mode.is_some() {
        base.appearance.mode = partial.appearance.mode;
    }
    // ... extend for other fields as needed
}

impl Default for ThemuxConfig {
    fn default() -> Self {
        Self {
            actions: Vec::new(),
            ui: None,
            new_workspace_command: None,
            commands: Vec::new(),
            vault: Vec::new(),
            shortcuts: std::collections::HashMap::new(),
            appearance: super::AppearanceConfig::default(),
            notifications: super::NotificationConfig::default(),
            terminal: super::TerminalConfig::default(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("IO error reading {0}: {1}")]
    Io(PathBuf, std::io::Error),
    #[error("Parse error in {0}: {1}")]
    Parse(PathBuf, serde_json::Error),
}
