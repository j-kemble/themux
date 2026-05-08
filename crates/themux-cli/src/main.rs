// themux-cli: Command-line interface for controlling themux.
//
// Communicates with the themux GUI via Unix domain socket using
// cmux's V2 JSON-RPC protocol. Compatible with cmux CLI usage patterns.

mod commands;
mod shell;

use clap::{Parser, Subcommand};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

/// themux: A native Linux terminal multiplexer for AI coding agents.
#[derive(Parser)]
#[command(name = "themux", version, about)]
pub struct Cli {
    /// Socket path (default: ~/.local/share/themux/themux.sock)
    #[arg(long, env = "THEMUX_SOCKET_PATH")]
    pub socket: Option<String>,

    /// Socket password
    #[arg(long, env = "THEMUX_SOCKET_PASSWORD")]
    pub password: Option<String>,

    /// Output format
    #[arg(long, default_value = "refs")]
    pub id_format: Option<String>,

    /// JSON output mode
    #[arg(long)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Ping the server
    Ping,

    /// Show version info
    Version,

    /// Get server capabilities
    Capabilities,

    /// Identify current context
    Identify,

    // --- Workspace commands ---
    /// List all workspaces
    #[command(alias = "list-workspaces")]
    WorkspaceList,
    /// Create a new workspace
    #[command(alias = "new-workspace")]
    WorkspaceCreate {
        #[arg(long)]
        cwd: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        command: Option<String>,
        #[arg(long)]
        focus: bool,
    },
    /// Select a workspace
    #[command(alias = "select-workspace")]
    WorkspaceSelect {
        workspace: String,
    },
    /// Close a workspace
    #[command(alias = "close-workspace")]
    WorkspaceClose {
        workspace: String,
    },

    // --- Surface commands ---
    /// Create a new surface (terminal or browser)
    #[command(alias = "new-surface")]
    SurfaceCreate {
        #[arg(long, default_value = "terminal")]
        r#type: String,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        focus: bool,
    },
    /// Send text to a surface
    Send {
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        surface: Option<String>,
        text: String,
    },
    /// Send a key to a surface
    #[command(alias = "send-key")]
    SendKey {
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        surface: Option<String>,
        key: String,
    },

    // --- Pane commands ---
    /// Create a new split pane
    #[command(alias = "new-pane")]
    NewPane {
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long, default_value = "horizontal")]
        direction: String,
        #[arg(long)]
        focus: bool,
    },

    // --- Browser commands ---
    /// Open a URL in the browser panel
    #[command(alias = "browser")]
    Browser {
        #[command(subcommand)]
        action: BrowserAction,
    },

    // --- Notification commands ---
    /// Send a notification
    Notify {
        #[arg(long)]
        title: String,
        #[arg(long)]
        subtitle: Option<String>,
        #[arg(long)]
        body: Option<String>,
    },

    // --- SSH ---
    /// Create a workspace on a remote host via SSH
    Ssh {
        #[arg(long)]
        destination: String,
        #[arg(long)]
        identity_file: Option<String>,
        #[arg(long, default_value = "22")]
        port: u16,
        #[arg(long)]
        cwd: Option<String>,
        #[arg(long)]
        name: Option<String>,
    },

    // --- Agent hooks ---
    /// Install agent hooks
    #[command(alias = "hooks")]
    Hooks {
        #[command(subcommand)]
        action: HooksAction,
    },

    // --- VM commands ---
    /// Cloud VM management
    #[command(alias = "vm")]
    Vm {
        #[command(subcommand)]
        action: VmAction,
    },

    // --- Session ---
    /// Restore the previous session
    #[command(alias = "restore-session")]
    RestoreSession,

    // --- Settings ---
    /// Open settings
    Settings {
        target: Option<String>,
    },

    // --- Themes ---
    /// Manage terminal themes
    Themes {
        #[command(subcommand)]
        action: ThemuxThemeAction,
    },

    // --- Tmux compatibility ---
    /// Pass-through tmux command (compatibility mode)
    #[command(hide = true)]
    TmuxCompat {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum BrowserAction {
    /// Navigate to URL
    Navigate { url: String },
    /// Open URL in a new browser split
    #[command(alias = "open-split")]
    OpenSplit { url: String },
    /// Go back
    Back,
    /// Go forward
    Forward,
    /// Reload page
    Reload,
    /// Get current URL
    GetUrl,
    /// Evaluate JavaScript
    Eval { code: String },
    /// Take screenshot
    Screenshot,
}

#[derive(Subcommand)]
pub enum HooksAction {
    /// Install hooks for an agent
    Install { agent: String },
    /// Uninstall hooks
    Uninstall { agent: String },
    /// List installed hooks
    List,
}

#[derive(Subcommand)]
pub enum VmAction {
    /// List cloud VMs
    #[command(alias = "ls")]
    List,
    /// Create a new cloud VM
    #[command(alias = "new")]
    Create {
        #[arg(long)]
        image: Option<String>,
        #[arg(long)]
        provider: Option<String>,
    },
    /// Attach to a cloud VM
    #[command(alias = "shell")]
    Attach { id: String },
    /// Destroy a cloud VM
    #[command(alias = "rm")]
    Destroy { id: String },
}

#[derive(Subcommand)]
pub enum ThemuxThemeAction {
    /// List available themes
    List,
    /// Set theme
    Set {
        theme: String,
        #[arg(long)]
        light: Option<String>,
        #[arg(long)]
        dark: Option<String>,
    },
    /// Clear theme override
    Clear,
}

fn main() {
    let cli = Cli::parse();

    // Resolve socket path
    let socket_path = cli.socket.clone().unwrap_or_else(|| {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
            .join("themux");
        data_dir.join("themux.sock").to_string_lossy().to_string()
    });

    if let Err(err) = dispatch_command(&cli, Path::new(&socket_path)) {
        eprintln!("themux: {err}");
        std::process::exit(1);
    }
}

fn dispatch_command(cli: &Cli, socket_path: &Path) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Ping => {
            let result = send_rpc(socket_path, cli.password.as_deref(), "1", "system.ping")?;
            print_result(result, cli.json)?;
        }
        Commands::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Capabilities => {
            let result =
                send_rpc(socket_path, cli.password.as_deref(), "1", "system.capabilities")?;
            print_capabilities(result, cli.json)?;
        }
        Commands::Identify => {
            let result = send_rpc(socket_path, cli.password.as_deref(), "1", "system.identify")?;
            print_result(result, cli.json)?;
        }
        _ => {
            println!("not implemented yet");
        }
    }

    Ok(())
}

fn send_rpc(
    socket_path: &Path,
    password: Option<&str>,
    id: &str,
    method: &str,
) -> anyhow::Result<Value> {
    let mut stream = UnixStream::connect(socket_path)
        .map_err(|err| anyhow::anyhow!("failed to connect to {}: {err}", socket_path.display()))?;

    if let Some(password) = password {
        writeln!(stream, "auth {password}")?;
        stream.flush()?;

        let mut reader = BufReader::new(stream.try_clone()?);
        let mut auth_response = String::new();
        reader.read_line(&mut auth_response)?;
        if auth_response.trim() != "OK" {
            anyhow::bail!("socket authentication failed: {}", auth_response.trim());
        }
    }

    let request = serde_json::json!({
        "id": id,
        "method": method,
        "params": {},
    });

    writeln!(stream, "{}", serde_json::to_string(&request)?)?;
    stream.flush()?;

    let mut reader = BufReader::new(stream);
    let mut response_line = String::new();
    let bytes_read = reader.read_line(&mut response_line)?;
    if bytes_read == 0 {
        anyhow::bail!("socket closed before sending a response");
    }

    let response: Value = serde_json::from_str(response_line.trim())?;
    if response.get("ok").and_then(Value::as_bool) == Some(false) {
        let code = response
            .pointer("/error/code")
            .and_then(Value::as_str)
            .unwrap_or("error");
        let message = response
            .pointer("/error/message")
            .and_then(Value::as_str)
            .unwrap_or("request failed");
        anyhow::bail!("{code}: {message}");
    }

    Ok(response.get("result").cloned().unwrap_or(response))
}

fn print_result(result: Value, json_output: bool) -> anyhow::Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else if let Some(text) = result.as_str() {
        println!("{text}");
    } else {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}

fn print_capabilities(result: Value, json_output: bool) -> anyhow::Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    if let Some(features) = result.get("features").and_then(Value::as_array) {
        for feature in features.iter().filter_map(Value::as_str) {
            println!("{feature}");
        }
    } else {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}
