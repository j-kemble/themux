// V2 JSON-RPC message types and dispatch.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A V2 JSON-RPC request.
#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: String,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// A V2 JSON-RPC success response.
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub id: String,
    pub ok: bool,
    pub result: Value,
}

/// A V2 JSON-RPC error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub id: String,
    pub ok: bool,
    pub error: RpcError,
}

#[derive(Debug, Serialize)]
pub struct RpcError {
    pub code: String,
    pub message: String,
}

/// V2 method dispatch.
pub async fn dispatch(request: Request) -> Result<Value, RpcError> {
    match request.method.as_str() {
        // System
        "system.ping" => Ok(serde_json::json!({"pong": true})),
        "system.identify" => Ok(serde_json::json!({
            "server": "themux",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "v2"
        })),
        "system.capabilities" => Ok(serde_json::json!({
            "protocols": ["v1", "v2"],
            "features": [
                "workspace.create", "workspace.list", "workspace.close",
                "pane.create", "surface.create", "surface.send_text",
                "browser.navigate", "browser.eval",
                "notification.create",
            ]
        })),

        // Workspace
        "workspace.list" => Ok(serde_json::json!({"workspaces": []})),
        "workspace.create" => Ok(serde_json::json!({"workspace_id": "stub"})),

        // Surface
        "surface.create" => Ok(serde_json::json!({"surface_id": "stub"})),
        "surface.send_text" => Ok(serde_json::json!({"sent": true})),

        // Unknown method
        _ => Err(RpcError {
            code: "method_not_found".into(),
            message: format!("unknown method: {}", request.method),
        }),
    }
}
