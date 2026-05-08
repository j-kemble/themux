// Agent hook installer.
//
// Creates tmux shim scripts in ~/.themux/agent-bin/<name>/tmux and
// ~/.themux/agent-bin/<name>/terminal-notifier. Sets CMUX_SOCKET_PATH,
// CMUX_WORKSPACE_ID, TMUX, TMUX_PANE env vars in wrapper scripts.

use crate::AgentKind;
use std::path::PathBuf;

/// Install hooks for a given agent.
pub fn install_hooks(agent: AgentKind) -> Result<(), HookError> {
    let bin_dir = hook_bin_dir(agent);
    std::fs::create_dir_all(&bin_dir).map_err(|e| HookError::Io(bin_dir.clone(), e))?;

    // Create tmux shim
    let tmux_shim = bin_dir.join("tmux");
    let tmux_content = tmux_shim_script();
    std::fs::write(&tmux_shim, &tmux_content)
        .map_err(|e| HookError::Io(tmux_shim.clone(), e))?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&tmux_shim)
            .map_err(|e| HookError::Io(tmux_shim.clone(), e))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&tmux_shim, perms)
            .map_err(|e| HookError::Io(tmux_shim.clone(), e))?;
    }

    // Create terminal-notifier shim
    let notifier_shim = bin_dir.join("terminal-notifier");
    let notifier_content = notifier_shim_script();
    std::fs::write(&notifier_shim, &notifier_content)
        .map_err(|e| HookError::Io(notifier_shim.clone(), e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&notifier_shim)
            .map_err(|e| HookError::Io(notifier_shim.clone(), e))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&notifier_shim, perms)
            .map_err(|e| HookError::Io(notifier_shim.clone(), e))?;
    }

    tracing::info!("Installed {} hooks in {:?}", agent_name(agent), bin_dir);
    Ok(())
}

/// Uninstall hooks for an agent.
pub fn uninstall_hooks(agent: AgentKind) -> Result<(), HookError> {
    let bin_dir = hook_bin_dir(agent);
    if bin_dir.exists() {
        std::fs::remove_dir_all(&bin_dir).map_err(|e| HookError::Io(bin_dir, e))?;
    }
    Ok(())
}

fn hook_bin_dir(agent: AgentKind) -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".themux").join("agent-bin").join(agent_dir_name(agent))
}

fn agent_dir_name(agent: AgentKind) -> &'static str {
    match agent {
        AgentKind::ClaudeCode | AgentKind::ClaudeTeams => "claude-teams",
        AgentKind::Codex => "omx",
        AgentKind::OpenCode => "omo",
        AgentKind::HermesAgent => "hermes-agent",
        AgentKind::RovoDev => "rovodev",
        AgentKind::Cursor => "cursor",
        AgentKind::Gemini => "gemini",
        AgentKind::Copilot => "copilot",
        AgentKind::CodeBuddy => "codebuddy",
        AgentKind::Factory => "factory",
        AgentKind::Custom => "custom",
    }
}

fn agent_name(agent: AgentKind) -> &'static str {
    match agent {
        AgentKind::ClaudeCode => "Claude Code",
        AgentKind::ClaudeTeams => "Claude Code Teams",
        AgentKind::Codex => "Codex",
        AgentKind::OpenCode => "OpenCode",
        AgentKind::HermesAgent => "Hermes Agent",
        AgentKind::RovoDev => "RovoDev",
        AgentKind::Cursor => "Cursor",
        AgentKind::Gemini => "Gemini",
        AgentKind::Copilot => "Copilot",
        AgentKind::CodeBuddy => "CodeBuddy",
        AgentKind::Factory => "Factory",
        AgentKind::Custom => "Custom",
    }
}

/// Generate the tmux shim script content.
fn tmux_shim_script() -> String {
    r#"#!/bin/sh
# themux tmux compatibility shim
# Proxies tmux commands to themux socket.
exec themux __tmux-compat "$@"
"#
    .to_string()
}

/// Generate the terminal-notifier shim script content.
fn notifier_shim_script() -> String {
    r#"#!/bin/sh
# themux notification shim
# Proxies terminal-notifier calls to themux notify.
THEMUX_TITLE=""
THEMUX_SUBTITLE=""
THEMUX_BODY=""

while [ $# -gt 0 ]; do
    case "$1" in
        -title) THEMUX_TITLE="$2"; shift 2 ;;
        -subtitle) THEMUX_SUBTITLE="$2"; shift 2 ;;
        -message) THEMUX_BODY="$2"; shift 2 ;;
        *) shift ;;
    esac
done

exec themux notify --title "$THEMUX_TITLE" \
    ${THEMUX_SUBTITLE:+--subtitle "$THEMUX_SUBTITLE"} \
    ${THEMUX_BODY:+--body "$THEMUX_BODY"}
"#
    .to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum HookError {
    #[error("IO error at {0}: {1}")]
    Io(PathBuf, std::io::Error),
}
