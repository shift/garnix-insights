//! Model Context Protocol (MCP) server implementation for Garnix Insights
//!
//! This module provides MCP server functionality for AI agents to query Garnix build status.
//! Uses a simple JSON-RPC 2.0 implementation over stdio.

use crate::client::GarnixClient;
use crate::error::{GarnixError, GarnixResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Negotiate a supported MCP protocol version from an optional selector
pub fn negotiate_version(requested: Option<&str>) -> McpVersion {
    if let Some(v) = requested.and_then(McpVersion::parse_selector) {
        if McpVersion::SUPPORTED.contains(&v) {
            return v;
        }
    }
    // default to stable when not specified or unsupported
    McpVersion::V2025_03_26
}

/// MCP server for Garnix Insights
pub struct GarnixMcpServer {
    client: GarnixClient,
    version: McpVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Supported MCP protocol versions (aligned with OpenCode)
pub enum McpVersion {
    /// Legacy protocol version
    V2024_11_05,
    /// Stable protocol version
    V2025_03_26,
    /// Latest protocol version
    V2025_06_18,
}

impl McpVersion {
    /// Ordered list of supported versions (preferred first)
    pub const SUPPORTED: &'static [McpVersion] = &[
        McpVersion::V2025_06_18,
        McpVersion::V2025_03_26,
        McpVersion::V2024_11_05,
    ];

    /// Parse a version selector (aliases allowed)
    pub fn parse_selector(s: &str) -> Option<Self> {
        match s {
            "latest" | "2025-06-18" => Some(Self::V2025_06_18),
            "stable" | "2025-03-26" => Some(Self::V2025_03_26),
            "legacy" | "2024-11-05" => Some(Self::V2024_11_05),
            _ => None,
        }
    }

    /// Return canonical date string for the version
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V2025_06_18 => "2025-06-18",
            Self::V2025_03_26 => "2025-03-26",
            Self::V2024_11_05 => "2024-11-05",
        }
    }
}

impl Default for GarnixMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl GarnixMcpServer {
    /// Create a new MCP server instance
    pub fn new() -> Self {
        Self {
            client: GarnixClient::new(),
            version: McpVersion::V2025_03_26,
        }
    }

    /// Create a new MCP server with custom Garnix client
    pub fn with_client(client: GarnixClient) -> Self {
        Self {
            client,
            version: McpVersion::V2025_03_26,
        }
    }

    /// Create a new MCP server with custom client and version
    pub fn with_client_and_version(client: GarnixClient, version: McpVersion) -> Self {
        Self { client, version }
    }

    /// Run the MCP server on stdio transport
    pub async fn run_stdio(self) -> GarnixResult<()> {
        tracing::info!("Starting Garnix Insights MCP server on stdio transport");

        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        // Send initial server info (announce negotiated/default version and supported list)
        let server_info = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "result": {
                "protocolVersion": self.version.as_str(),
                "serverInfo": {
                    "name": "garnix-insights",
                    "version": "0.2.0"
                },
                "capabilities": {
                    "tools": {
                        "listChanged": false
                    }
                },
                "supportedVersions": McpVersion::SUPPORTED.iter().map(|v| v.as_str()).collect::<Vec<_>>()
            }
        });

        stdout
            .write_all(format!("{}\n", server_info).as_bytes())
            .await
            .map_err(|e| GarnixError::NetworkError(format!("Failed to write to stdout: {}", e)))?;
        stdout
            .flush()
            .await
            .map_err(|e| GarnixError::NetworkError(format!("Failed to flush stdout: {}", e)))?;

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    tracing::info!("MCP server received EOF, shutting down");
                    break;
                }
                Ok(_) => {
                    if let Ok(request) = serde_json::from_str::<McpRequest>(line.trim()) {
                        let response = self.handle_request(request).await;
                        let response_json = serde_json::to_string(&response).unwrap_or_default();

                        stdout
                            .write_all(format!("{}\n", response_json).as_bytes())
                            .await
                            .map_err(|e| {
                                GarnixError::NetworkError(format!(
                                    "Failed to write response: {}",
                                    e
                                ))
                            })?;
                        stdout.flush().await.map_err(|e| {
                            GarnixError::NetworkError(format!("Failed to flush stdout: {}", e))
                        })?;
                    } else {
                        tracing::warn!("Invalid JSON-RPC request: {}", line.trim());
                    }
                }
                Err(e) => {
                    tracing::error!("Error reading from stdin: {}", e);
                    return Err(GarnixError::NetworkError(format!("stdin error: {}", e)));
                }
            }
        }

        Ok(())
    }

    async fn handle_request(&self, request: McpRequest) -> McpResponse {
        match request.method.as_str() {
            "initialize" => {
                tracing::info!("Handling initialize request");
                // Allow client to request a version in params.protocolVersion
                let requested = request
                    .params
                    .as_ref()
                    .and_then(|p| p.get("protocolVersion"))
                    .and_then(|v| v.as_str());
                let chosen = negotiate_version(requested);

                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({
                        "protocolVersion": chosen.as_str(),
                        "serverInfo": {
                            "name": "garnix-insights",
                            "version": "0.2.0"
                        },
                        "capabilities": {
                            "tools": {
                                "listChanged": false
                            }
                        },
                        "supportedVersions": McpVersion::SUPPORTED.iter().map(|v| v.as_str()).collect::<Vec<_>>()
                    })),
                    error: None,
                }
            }
            "tools/list" => {
                tracing::info!("Handling tools/list request");
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({
                        "tools": [
                            {
                                "name": "get_build_status",
                                "description": "Get the build status for a specific commit from Garnix",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "commit_id": {
                                            "type": "string",
                                            "description": "The commit SHA to check build status for"
                                        },
                                        "token": {
                                            "type": "string",
                                            "description": "JWT token for Garnix API authentication"
                                        }
                                    },
                                    "required": ["commit_id", "token"]
                                }
                            },
                            {
                                "name": "get_build_logs",
                                "description": "Get detailed build logs for a specific commit from Garnix",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "commit_id": {
                                            "type": "string",
                                            "description": "The commit SHA to get logs for"
                                        },
                                        "token": {
                                            "type": "string",
                                            "description": "JWT token for Garnix API authentication"
                                        }
                                    },
                                    "required": ["commit_id", "token"]
                                }
                            },
                            {
                                "name": "check_commit_ready",
                                "description": "Check if a commit is ready for deployment (all builds passed)",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "commit_id": {
                                            "type": "string",
                                            "description": "The commit SHA to check readiness for"
                                        },
                                        "token": {
                                            "type": "string",
                                            "description": "JWT token for Garnix API authentication"
                                        }
                                    },
                                    "required": ["commit_id", "token"]
                                }
                            }
                        ]
                    })),
                    error: None,
                }
            }
            "tools/call" => {
                tracing::info!("Handling tools/call request");
                match self.handle_tool_call(request.params).await {
                    Ok(result) => McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(result),
                        error: None,
                    },
                    Err(error_msg) => McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(McpError {
                            code: -32000,
                            message: error_msg,
                            data: None,
                        }),
                    },
                }
            }
            _ => {
                tracing::warn!("Unknown method: {}", request.method);
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(McpError {
                        code: -32601,
                        message: "Method not found".to_string(),
                        data: None,
                    }),
                }
            }
        }
    }

    async fn handle_tool_call(&self, params: Option<Value>) -> Result<Value, String> {
        let params = params.ok_or("Missing parameters for tool call")?;
        let tool_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing tool name")?;
        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        match tool_name {
            "get_build_status" => self.handle_get_build_status(arguments).await,
            "get_build_logs" => self.handle_get_build_logs(arguments).await,
            "check_commit_ready" => self.handle_check_commit_ready(arguments).await,
            _ => Err(format!("Unknown tool: {}", tool_name)),
        }
    }

    async fn handle_get_build_status(&self, arguments: Value) -> Result<Value, String> {
        let commit_id = arguments
            .get("commit_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing required argument: commit_id")?;

        let token = arguments
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or("Missing required argument: token")?;

        match self.client.fetch_build_status(token, commit_id).await {
            Ok(status) => {
                let status_json = serde_json::to_value(&status)
                    .map_err(|e| format!("Failed to serialize status: {}", e))?;
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Build Status for commit {}:\n\n{}", commit_id,
                            serde_json::to_string_pretty(&status_json).unwrap_or_default())
                    }]
                }))
            }
            Err(e) => Err(format!("Error getting build status: {}", e)),
        }
    }

    async fn handle_get_build_logs(&self, arguments: Value) -> Result<Value, String> {
        let commit_id = arguments
            .get("commit_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing required argument: commit_id")?;

        let token = arguments
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or("Missing required argument: token")?;

        match self.client.fetch_build_status(token, commit_id).await {
            Ok(status) => {
                let logs_text = status.builds
                    .iter()
                    .map(|build| {
                        format!(
                            "**Build {} ({})**:\nStatus: {}\nSystem: {}\nPackage: {}\nDuration: {} to {}",
                            build.id,
                            build.system.as_ref().unwrap_or(&"unknown".to_string()),
                            build.status_with_emoji(),
                            build.system.as_ref().unwrap_or(&"unknown".to_string()),
                            build.package,
                            build.start_time,
                            build.end_time
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");

                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Build Information for commit {}:\n\n{}", commit_id, logs_text)
                    }]
                }))
            }
            Err(e) => Err(format!("Error getting build logs: {}", e)),
        }
    }

    async fn handle_check_commit_ready(&self, arguments: Value) -> Result<Value, String> {
        let commit_id = arguments
            .get("commit_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing required argument: commit_id")?;

        let token = arguments
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or("Missing required argument: token")?;

        match self.client.fetch_build_status(token, commit_id).await {
            Ok(status) => {
                let total_builds = status.builds.len() as u32;
                let success_rate = status.success_rate();
                let is_ready = success_rate == 100.0 && total_builds > 0;

                let status_text = if is_ready {
                    format!(
                        "[OK] Commit {} is ready for deployment! All {} builds passed.",
                        commit_id, total_builds
                    )
                } else if total_builds == 0 {
                    format!("[PENDING] Commit {} has no builds yet.", commit_id)
                } else {
                    format!(
                        "[FAIL] Commit {} is NOT ready. {}/{} builds passed ({:.1}% success rate).",
                        commit_id, status.summary.succeeded, total_builds, success_rate
                    )
                };

                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": status_text
                    }]
                }))
            }
            Err(e) => Err(format!("Error checking commit readiness: {}", e)),
        }
    }
}

/// JSON-RPC 2.0 request structure for MCP
#[derive(Debug, Deserialize)]
struct McpRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// JSON-RPC 2.0 response structure for MCP
#[derive(Debug, Serialize)]
struct McpResponse {
    jsonrpc: String,
    id: Option<Value>,
    result: Option<Value>,
    error: Option<McpError>,
}

/// JSON-RPC 2.0 error structure
#[derive(Debug, Serialize)]
struct McpError {
    code: i32,
    message: String,
    data: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = GarnixMcpServer::new();
        // Just test that it can be created
        drop(server);
    }

    #[test]
    fn test_server_with_client() {
        let client = GarnixClient::new();
        let server = GarnixMcpServer::with_client(client);
        drop(server);
    }

    #[test]
    fn test_default_implementation() {
        let server1 = GarnixMcpServer::new();
        let server2 = GarnixMcpServer::default();

        // Both should be created successfully
        drop(server1);
        drop(server2);
    }

    #[test]
    fn test_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let request: Result<McpRequest, _> = serde_json::from_str(json);
        assert!(request.is_ok());

        let req = request.unwrap();
        assert_eq!(req.method, "initialize");
        assert_eq!(req.jsonrpc, "2.0");
    }

    #[tokio::test]
    async fn test_tool_call_missing_params() {
        let client = GarnixClient::new();
        let server = GarnixMcpServer::with_client(client);

        let result = server.handle_tool_call(None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing parameters"));
    }
}
