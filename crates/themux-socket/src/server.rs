// Unix domain socket server: listener, auth, connection handling.

use std::path::PathBuf;
use tokio::net::UnixListener;

/// The socket server that accepts client connections and dispatches
/// V2 JSON-RPC requests to the application core.
pub struct SocketServer {
    socket_path: PathBuf,
    password: Option<String>,
    listener: Option<UnixListener>,
}

impl SocketServer {
    /// Create a new socket server.
    pub fn new(socket_path: PathBuf, password: Option<String>) -> Self {
        Self {
            socket_path,
            password,
            listener: None,
        }
    }

    /// Start listening on the Unix socket.
    pub async fn bind(&mut self) -> Result<(), std::io::Error> {
        // Remove stale socket file
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        // Ensure parent directory exists
        if let Some(parent) = self.socket_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let listener = UnixListener::bind(&self.socket_path)?;
        self.listener = Some(listener);

        tracing::info!("Socket server listening on {:?}", self.socket_path);
        Ok(())
    }

    /// Accept connections in a loop, spawning a task for each.
    pub async fn serve(&self) -> Result<(), std::io::Error> {
        let listener = self.listener.as_ref().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotConnected, "not bound")
        })?;

        loop {
            let (stream, addr) = listener.accept().await?;
            tracing::debug!("New connection from {:?}", addr);

            let password = self.password.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, password.as_deref()).await {
                    tracing::error!("Connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(
    stream: tokio::net::UnixStream,
    password: Option<&str>,
) -> Result<(), std::io::Error> {
    use crate::protocol::v2;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Read initial line (V1 auth or direct V2 JSON)
    if reader.read_line(&mut line).await? == 0 {
        return Ok(());
    }

    // If password is required and this looks like an auth attempt
    if let Some(expected) = password {
        if let Some(provided) = line.strip_prefix("auth ") {
            if provided.trim() == expected {
                writer.write_all(b"OK\n").await?;
                line.clear();
                // Read next line after auth
                if reader.read_line(&mut line).await? == 0 {
                    return Ok(());
                }
            } else {
                writer.write_all(b"ERROR: Access denied\n").await?;
                return Ok(());
            }
        }
    }

    // Dispatch V2 JSON-RPC loop
    loop {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            match serde_json::from_str::<v2::Request>(trimmed) {
                Ok(req) => {
                    let id = req.id.clone();
                    match v2::dispatch(req).await {
                        Ok(result) => {
                            let resp = v2::SuccessResponse {
                                id,
                                ok: true,
                                result,
                            };
                            let json = serde_json::to_string(&resp).unwrap();
                            writer.write_all(json.as_bytes()).await?;
                            writer.write_all(b"\n").await?;
                        }
                        Err(error) => {
                            let resp = v2::ErrorResponse {
                                id,
                                ok: false,
                                error,
                            };
                            let json = serde_json::to_string(&resp).unwrap();
                            writer.write_all(json.as_bytes()).await?;
                            writer.write_all(b"\n").await?;
                        }
                    }
                }
                Err(e) => {
                    // Try to extract ID for error response if possible
                    let id = serde_json::from_str::<serde_json::Value>(trimmed)
                        .ok()
                        .and_then(|v| {
                            v.get("id")
                                .and_then(|id| id.as_str().map(|s| s.to_string()))
                        })
                        .unwrap_or_else(|| "null".to_string());

                    let resp = v2::ErrorResponse {
                        id,
                        ok: false,
                        error: v2::RpcError {
                            code: "parse_error".into(),
                            message: e.to_string(),
                        },
                    };
                    let json = serde_json::to_string(&resp).unwrap();
                    writer.write_all(json.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            }
        }

        line.clear();
        if reader.read_line(&mut line).await? == 0 {
            break; // EOF
        }
    }

    Ok(())
}

impl Drop for SocketServer {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket_path);
    }
}
