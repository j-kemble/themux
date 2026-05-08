// Keyboard shortcut settings.
//
// Defines the 80+ configurable actions from cmux and their default shortcuts.

use serde::{Deserialize, Serialize};

/// All configurable keyboard shortcut actions (cmux-compatible).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutAction {
    // Workspaces
    NewWorkspace,
    JumpToWorkspace1,
    JumpToWorkspace2,
    JumpToWorkspace3,
    JumpToWorkspace4,
    JumpToWorkspace5,
    JumpToWorkspace6,
    JumpToWorkspace7,
    JumpToWorkspace8,
    JumpToLastWorkspace,
    NextWorkspace,
    PreviousWorkspace,
    CloseWorkspace,
    RenameWorkspace,
    EditWorkspaceDescription,
    ToggleSidebar,
    FocusRightSidebar,

    // Surfaces
    NewSurface,
    NextSurface,
    PreviousSurface,
    JumpToSurface1,
    JumpToSurface2,
    JumpToSurface3,
    JumpToSurface4,
    JumpToSurface5,
    JumpToSurface6,
    JumpToSurface7,
    JumpToSurface8,
    JumpToLastSurface,
    CloseSurface,

    // Split Panes
    SplitRight,
    SplitDown,
    FocusPaneLeft,
    FocusPaneRight,
    FocusPaneUp,
    FocusPaneDown,
    FlashFocusedPanel,

    // Browser
    OpenBrowser,
    FocusAddressBar,
    BrowserBack,
    BrowserForward,
    BrowserReload,
    ToggleDevTools,
    ShowJsConsole,

    // Notifications
    ShowNotifications,
    JumpToLatestUnread,

    // Find
    Find,
    FindNext,
    FindPrevious,
    HideFind,
    UseSelectionForFind,

    // Terminal
    ClearScrollback,
    Copy,
    Paste,
    IncreaseFontSize,
    DecreaseFontSize,
    ResetFontSize,

    // Window
    NewWindow,
    ReopenPreviousSession,
    OpenSettings,
    ReloadConfiguration,
    Quit,

    // Command palette
    OpenCommandPalette,
}

impl ShortcutAction {
    /// Default shortcut for each action (matching cmux macOS defaults,
    /// mapped to Linux key names using Super instead of Cmd).
    pub fn default_shortcut(&self) -> Option<(&str, &[&str])> {
        use ShortcutAction::*;
        match self {
            NewWorkspace => Some(("n", &["Super"])),
            CloseWorkspace => Some(("w", &["Super", "Shift"])),
            NewSurface => Some(("t", &["Super"])),
            CloseSurface => Some(("w", &["Super"])),
            SplitRight => Some(("d", &["Super"])),
            SplitDown => Some(("d", &["Super", "Shift"])),
            FocusPaneLeft => Some(("Left", &["Super", "Alt"])),
            FocusPaneRight => Some(("Right", &["Super", "Alt"])),
            FocusPaneUp => Some(("Up", &["Super", "Alt"])),
            FocusPaneDown => Some(("Down", &["Super", "Alt"])),
            ToggleSidebar => Some(("b", &["Super"])),
            Find => Some(("f", &["Super"])),
            Copy => Some(("c", &["Super"])),
            Paste => Some(("v", &["Super"])),
            ClearScrollback => Some(("k", &["Super"])),
            IncreaseFontSize => Some(("plus", &["Super"])),
            DecreaseFontSize => Some(("minus", &["Super"])),
            ResetFontSize => Some(("0", &["Super"])),
            OpenSettings => Some(("comma", &["Super"])),
            ReloadConfiguration => Some(("comma", &["Super", "Shift"])),
            Quit => Some(("q", &["Super"])),
            OpenCommandPalette => Some(("p", &["Super", "Shift"])),
            NewWindow => Some(("n", &["Super", "Shift"])),
            BrowserBack => Some(("bracketleft", &["Super"])),
            BrowserForward => Some(("bracketright", &["Super"])),
            BrowserReload => Some(("r", &["Super"])),
            // Default: no shortcut
            _ => None,
        }
    }
}
