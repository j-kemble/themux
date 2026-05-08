// Agent vault: custom agent registration from config.

use crate::AgentKind;
use serde::{Deserialize, Serialize};

/// A registered agent from the vault config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultAgent {
    pub name: String,
    pub kind: String,
    pub executable: String,
    pub working_directory_policy: WorkingDirPolicy,
    pub environment: std::collections::HashMap<String, String>,
    pub install_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkingDirPolicy {
    UseProjectDir,
    UseSpecificDir(String),
    Ignore,
}

impl VaultAgent {
    pub fn agent_kind(&self) -> Option<AgentKind> {
        AgentKind::from_str(&self.kind)
    }
}
