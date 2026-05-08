// themux-agent: AI coding agent hook system.
//
// Provides tmux compatibility shims so Claude Code, Codex, OpenCode,
// Hermes Agent, and other tools see themux as a tmux session.
// Agent hooks inject shim binaries into PATH, set fake TMUX env vars,
// and proxy tmux commands to the themux socket.

pub mod agents;
pub mod hooks;
pub mod vault;

/// Supported AI coding agent kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentKind {
    ClaudeCode,
    ClaudeTeams,
    Codex,
    OpenCode,
    HermesAgent,
    RovoDev,
    Cursor,
    Gemini,
    Copilot,
    CodeBuddy,
    Factory,
    Custom,
}

impl AgentKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "claude" | "claude-code" | "claude_code" => Some(Self::ClaudeCode),
            "claude-teams" | "claude_teams" => Some(Self::ClaudeTeams),
            "codex" => Some(Self::Codex),
            "opencode" | "open-code" | "open_code" | "omo" => Some(Self::OpenCode),
            "hermes" | "hermes-agent" | "hermes_agent" => Some(Self::HermesAgent),
            "rovodev" | "rovo-dev" | "rovo_dev" => Some(Self::RovoDev),
            "cursor" => Some(Self::Cursor),
            "gemini" => Some(Self::Gemini),
            "copilot" => Some(Self::Copilot),
            "codebuddy" | "code-buddy" | "code_buddy" => Some(Self::CodeBuddy),
            "factory" => Some(Self::Factory),
            _ => None,
        }
    }
}
