// SSH remote workspace: daemon bootstrap, tunnel management.

use super::RemoteProxyConfig;

/// Build the SSH command for bootstrapping cmuxd-remote on the target host.
pub fn build_ssh_bootstrap_command(
    config: &RemoteProxyConfig,
    remote_workdir: &str,
) -> Vec<String> {
    let mut cmd = vec![
        "ssh".to_string(),
        "-o".to_string(),
        "ConnectTimeout=6".to_string(),
        "-o".to_string(),
        "ServerAliveInterval=20".to_string(),
        "-o".to_string(),
        "ServerAliveCountMax=2".to_string(),
        "-o".to_string(),
        "StrictHostKeyChecking=accept-new".to_string(),
        "-o".to_string(),
        "BatchMode=yes".to_string(),
        "-p".to_string(),
        config.port.to_string(),
    ];

    if let Some(ref identity) = config.identity_file {
        cmd.push("-i".to_string());
        cmd.push(identity.clone());
    }

    cmd.push(config.host.clone());
    cmd.push("-T".to_string());
    cmd.push(format!(
        "cd {} && exec cmuxd-remote serve --stdio",
        remote_workdir
    ));

    cmd
}

/// Build the SSH command for port forwarding to the remote daemon.
pub fn build_ssh_forward_command(
    config: &RemoteProxyConfig,
    local_port: u16,
    remote_socket_path: &str,
) -> Vec<String> {
    let mut cmd = vec![
        "ssh".to_string(),
        "-N".to_string(),
        "-T".to_string(),
        "-o".to_string(),
        "ConnectTimeout=6".to_string(),
        "-o".to_string(),
        "ServerAliveInterval=20".to_string(),
        "-o".to_string(),
        "StrictHostKeyChecking=accept-new".to_string(),
        "-p".to_string(),
        config.port.to_string(),
        "-L".to_string(),
        format!("127.0.0.1:{}:{}", local_port, remote_socket_path),
    ];

    if let Some(ref identity) = config.identity_file {
        cmd.push("-i".to_string());
        cmd.push(identity.clone());
    }

    cmd.push(config.host.clone());
    cmd
}
