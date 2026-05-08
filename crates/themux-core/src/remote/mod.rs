// Remote workspace transport: SSH and WebSocket connections.
//
// Manages the lifecycle of remote daemon proxy tunnels, port forwarding,
// and reconnection with exponential backoff.

pub mod ssh;

use serde::{Deserialize, Serialize};

/// Remote proxy broker state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteProxyConfig {
    pub host: String,
    pub port: u16,
    pub identity_file: Option<String>,
    pub daemon_version: String,
}
