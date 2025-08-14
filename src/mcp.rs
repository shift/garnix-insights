//! Model Context Protocol (MCP) server implementation for Garnix Fetcher
//!
//! This module provides MCP server functionality but is currently disabled
//! due to API incompatibilities. It will be enabled once the proper rmcp version is available.

use crate::client::GarnixClient;
use crate::error::GarnixResult;

/// MCP server for Garnix Fetcher (currently disabled)
pub struct GarnixMcpServer {
    _client: GarnixClient,
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
            _client: GarnixClient::new(),
        }
    }

    /// Create a new MCP server with custom Garnix client
    pub fn with_client(client: GarnixClient) -> Self {
        Self { _client: client }
    }

    /// Run the MCP server on stdio transport (currently disabled)
    pub async fn run_stdio(self) -> GarnixResult<()> {
        use crate::error::GarnixError;
        
        tracing::warn!("MCP server functionality is currently disabled");
        tracing::warn!("This feature will be available in a future version");
        
        Err(GarnixError::ConfigError(
            "MCP server functionality is currently disabled".to_string()
        ))
    }
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

    #[tokio::test]
    async fn test_run_stdio_returns_error() {
        let server = GarnixMcpServer::new();
        
        // This should return an error since MCP is currently disabled
        let result = server.run_stdio().await;
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("MCP server functionality is currently disabled"));
        }
    }
}
