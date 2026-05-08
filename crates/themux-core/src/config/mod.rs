// Configuration system: cmux.json loading, validation, and types.
//
// Compatible with cmux's ~/.config/cmux/cmux.json schema, stored at
// ~/.config/themux/config.json on Linux.

pub mod loader;
pub mod schema;
pub mod shortcuts;

use serde::{Deserialize, Serialize};

/// Root configuration struct, compatible with cmux.json schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemuxConfig {
    /// Custom actions: command palette entries with keyboard shortcuts.
    #[serde(default)]
    pub actions: Vec<ConfigAction>,

    /// UI overrides.
    #[serde(default)]
    pub ui: Option<UiOverrides>,

    /// Default command for new workspaces.
    pub new_workspace_command: Option<String>,

    /// Commands: named workspace command definitions.
    #[serde(default)]
    pub commands: Vec<ConfigCommand>,

    /// Agent vault registrations.
    #[serde(default)]
    pub vault: Vec<VaultAgentConfig>,

    /// Keyboard shortcut overrides.
    #[serde(default)]
    pub shortcuts: std::collections::HashMap<String, StoredShortcut>,

    /// Appearance settings.
    #[serde(default)]
    pub appearance: AppearanceConfig,

    /// Notification settings.
    #[serde(default)]
    pub notifications: NotificationConfig,

    /// Terminal settings (compatible with Ghostty config).
    #[serde(default)]
    pub terminal: TerminalConfig,
}

/// A named custom action for the command palette.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAction {
    pub name: String,
    pub shortcut: Option<StoredShortcut>,
    pub command: String,
    pub description: Option<String>,
}

/// A named workspace command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCommand {
    pub name: String,
    pub command: String,
    pub description: Option<String>,
    #[serde(default)]
    pub confirm: bool,
    pub open_in: Option<OpenLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenLocation {
    NewWindow,
    NewWorkspace,
    NewPane,
    CurrentPane,
}

/// A registered AI agent for the vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultAgentConfig {
    pub name: String,
    pub executable: String,
    pub working_directory_policy: Option<String>,
    pub environment: Option<std::collections::HashMap<String, String>>,
    pub install_command: Option<String>,
}

/// Keyboard shortcut definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredShortcut {
    pub key: String,
    pub modifiers: Vec<String>,
}

/// UI override settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiOverrides {
    pub surface_tab_bar_buttons: Option<Vec<String>>,
}

/// Appearance configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub mode: Option<String>,  // system, light, dark
    pub theme: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
}

/// Notification configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: Option<bool>,
    pub sound: Option<String>,
    pub custom_command: Option<String>,
}

/// Terminal configuration (Ghostty-compatible subset).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub scrollback_limit: Option<usize>,
    pub unfocused_split_opacity: Option<f64>,
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub cursor_color: Option<String>,
    pub selection_color: Option<String>,
    pub palette: Option<Vec<String>>,
}
