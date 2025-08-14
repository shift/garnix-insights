//! HTTP client for interacting with the Garnix.io API

use crate::error::GarnixError;
use crate::types::{GarnixResponse, LogResponse};
use reqwest::{Client, StatusCode};
use tracing::{error, info, warn};

/// HTTP client for Garnix.io API
pub struct GarnixClient {
    client: Client,
    base_url: String,
}

impl Default for GarnixClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GarnixClient {
    /// Create a new Garnix client with default settings
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://garnix.io/api".to_string(),
        }
    }

    /// Create a new Garnix client with custom base URL
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Create a new Garnix client with custom HTTP client
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            base_url: "https://garnix.io/api".to_string(),
        }
    }

    /// Create a new Garnix client with custom HTTP client and base URL
    pub fn with_client_and_url(client: Client, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into(),
        }
    }

    /// Fetch build status for a specific commit
    ///
    /// # Arguments
    /// * `jwt_token` - JWT authentication token for Garnix.io API
    /// * `commit_id` - Git commit hash to fetch build status for
    ///
    /// # Returns
    /// Returns a `GarnixResponse` containing build summary and individual build details
    ///
    /// # Errors
    /// Returns `GarnixError` on network errors, authentication failures, or API errors
    pub async fn fetch_build_status(
        &self,
        jwt_token: &str,
        commit_id: &str,
    ) -> Result<GarnixResponse, GarnixError> {
        info!("Fetching build status for commit: {}", commit_id);

        let url = format!("{}/builds/{}", self.base_url, commit_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", jwt_token))
            .send()
            .await
            .map_err(|e| {
                error!("Network error fetching build status: {}", e);
                GarnixError::NetworkError(e.to_string())
            })?;

        let status_code = response.status();
        info!("API response status: {}", status_code);

        match status_code {
            StatusCode::OK => {
                let garnix_response = response.json::<GarnixResponse>().await.map_err(|e| {
                    error!("Failed to parse JSON response: {}", e);
                    GarnixError::ParseError(e.to_string())
                })?;

                info!("Successfully fetched build status for commit {}", commit_id);
                Ok(garnix_response)
            }
            StatusCode::UNAUTHORIZED => {
                error!("Authentication failed - invalid JWT token");
                Err(GarnixError::AuthenticationError(
                    "Invalid JWT token".to_string(),
                ))
            }
            StatusCode::NOT_FOUND => {
                error!("Commit not found: {}", commit_id);
                Err(GarnixError::NotFound(format!(
                    "Commit {} not found",
                    commit_id
                )))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                warn!("Rate limited by Garnix API");
                Err(GarnixError::RateLimit("Rate limit exceeded".to_string()))
            }
            _ => {
                let error_text = response.text().await.unwrap_or_default();
                error!("API error {}: {}", status_code, error_text);
                Err(GarnixError::ApiError(format!(
                    "HTTP {}: {}",
                    status_code, error_text
                )))
            }
        }
    }

    /// Fetch build logs for a specific build
    ///
    /// # Arguments
    /// * `jwt_token` - JWT authentication token for Garnix.io API
    /// * `build_id` - Unique build ID to fetch logs for
    ///
    /// # Returns
    /// Returns a `LogResponse` containing log entries and completion status
    ///
    /// # Errors
    /// Returns `GarnixError` on network errors, authentication failures, or API errors
    pub async fn fetch_build_logs(
        &self,
        jwt_token: &str,
        build_id: &str,
    ) -> Result<LogResponse, GarnixError> {
        info!("Fetching build logs for build: {}", build_id);

        let url = format!("{}/builds/{}/logs", self.base_url, build_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", jwt_token))
            .send()
            .await
            .map_err(|e| {
                error!("Network error fetching build logs: {}", e);
                GarnixError::NetworkError(e.to_string())
            })?;

        let status_code = response.status();
        info!("Logs API response status: {}", status_code);

        match status_code {
            StatusCode::OK => {
                let log_response = response.json::<LogResponse>().await.map_err(|e| {
                    error!("Failed to parse logs JSON response: {}", e);
                    GarnixError::ParseError(e.to_string())
                })?;

                info!(
                    "Successfully fetched {} log entries for build {}",
                    log_response.logs.len(),
                    build_id
                );
                Ok(log_response)
            }
            StatusCode::UNAUTHORIZED => {
                error!("Authentication failed - invalid JWT token");
                Err(GarnixError::AuthenticationError(
                    "Invalid JWT token".to_string(),
                ))
            }
            StatusCode::NOT_FOUND => {
                error!("Build not found: {}", build_id);
                Err(GarnixError::NotFound(format!(
                    "Build {} not found",
                    build_id
                )))
            }
            StatusCode::TOO_MANY_REQUESTS => {
                warn!("Rate limited by Garnix API");
                Err(GarnixError::RateLimit("Rate limit exceeded".to_string()))
            }
            _ => {
                let error_text = response.text().await.unwrap_or_default();
                error!("Logs API error {}: {}", status_code, error_text);
                Err(GarnixError::ApiError(format!(
                    "HTTP {}: {}",
                    status_code, error_text
                )))
            }
        }
    }

    /// Check if the API is accessible with the given token
    ///
    /// # Arguments
    /// * `jwt_token` - JWT authentication token to validate
    ///
    /// # Returns
    /// Returns `Ok(())` if the token is valid, `Err(GarnixError)` otherwise
    pub async fn validate_token(&self, jwt_token: &str) -> Result<(), GarnixError> {
        info!("Validating JWT token");

        let url = format!("{}/user", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", jwt_token))
            .send()
            .await
            .map_err(|e| {
                error!("Network error validating token: {}", e);
                GarnixError::NetworkError(e.to_string())
            })?;

        match response.status() {
            StatusCode::OK => {
                info!("JWT token is valid");
                Ok(())
            }
            StatusCode::UNAUTHORIZED => {
                error!("Invalid JWT token");
                Err(GarnixError::AuthenticationError(
                    "Invalid JWT token".to_string(),
                ))
            }
            status => {
                let error_text = response.text().await.unwrap_or_default();
                error!("Token validation error {}: {}", status, error_text);
                Err(GarnixError::ApiError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )))
            }
        }
    }

    /// Get the base URL for the API
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Build;
    use mockito::Server;
    use serde_json::json;
    use std::collections::HashMap;

    fn create_test_build() -> Build {
        Build {
            id: "test-build-1".to_string(),
            repo_user: "testuser".to_string(),
            repo_name: "testrepo".to_string(),
            branch: "main".to_string(),
            repo_is_public: true,
            git_commit: "abc123".to_string(),
            package: "test-package".to_string(),
            package_type: "derivation".to_string(),
            system: Some("x86_64-linux".to_string()),
            req_user: "testuser".to_string(),
            status: "Success".to_string(),
            start_time: "2024-01-01T00:00:00Z".to_string(),
            end_time: "2024-01-01T00:01:00Z".to_string(),
            drv_path: Some("/nix/store/test.drv".to_string()),
            output_paths: Some(HashMap::from([(
                "out".to_string(),
                "/nix/store/test".to_string(),
            )])),
            github_run_id: 12345,
            wants_incrementalism: false,
            eval_host: "eval.garnix.io".to_string(),
            uploaded_to_cache: true,
        }
    }

    #[tokio::test]
    async fn test_fetch_build_status_success() {
        let mut server = Server::new_async().await;
        let client = GarnixClient::with_base_url(server.url());

        let mock_response = json!({
            "summary": {
                "repo_owner": "testuser",
                "repo_name": "testrepo",
                "repo_is_public": true,
                "git_commit": "3b8e1f2a9c5d7e4a6b2f9e7c1a4d8f3a5c2e9b7d",
                "branch": "main",
                "req_user": "testuser",
                "start_time": "2024-01-01T00:00:00Z",
                "succeeded": 1,
                "failed": 0,
                "pending": 0,
                "cancelled": 0
            },
            "builds": [create_test_build()],
            "runs": []
        });

        let _mock = server
            .mock("GET", "/builds/3b8e1f2a9c5d7e4a6b2f9e7c1a4d8f3a5c2e9b7d")
            .match_header("authorization", "Bearer test-token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let result = client.fetch_build_status("test-token", "3b8e1f2a9c5d7e4a6b2f9e7c1a4d8f3a5c2e9b7d").await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.summary.git_commit, "3b8e1f2a9c5d7e4a6b2f9e7c1a4d8f3a5c2e9b7d");
        assert_eq!(response.builds.len(), 1);
    }

    #[tokio::test]
    async fn test_fetch_build_status_unauthorized() {
        let mut server = Server::new_async().await;
        let client = GarnixClient::with_base_url(server.url());

        let _mock = server
            .mock("GET", "/builds/4c9f2e7b1a5d8e3a6b1f4e9c2a7d5f8a3c6e1b9f")
            .match_header("authorization", "Bearer invalid-token")
            .with_status(401)
            .create_async()
            .await;

        let result = client.fetch_build_status("invalid-token", "4c9f2e7b1a5d8e3a6b1f4e9c2a7d5f8a3c6e1b9f").await;
        assert!(matches!(result, Err(GarnixError::AuthenticationError(_))));
    }

    #[tokio::test]
    async fn test_fetch_build_status_not_found() {
        let mut server = Server::new_async().await;
        let client = GarnixClient::with_base_url(server.url());

        let _mock = server
            .mock("GET", "/builds/nonexistent")
            .match_header("authorization", "Bearer test-token")
            .with_status(404)
            .create_async()
            .await;

        let result = client.fetch_build_status("test-token", "nonexistent").await;
        assert!(matches!(result, Err(GarnixError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_fetch_build_logs_success() {
        let mut server = Server::new_async().await;
        let client = GarnixClient::with_base_url(server.url());

        let mock_response = json!({
            "finished": true,
            "logs": [
                {
                    "timestamp": "2024-01-01T00:00:00Z",
                    "log_message": "Starting build..."
                },
                {
                    "timestamp": "2024-01-01T00:00:30Z",
                    "log_message": "Build completed successfully"
                }
            ]
        });

        let _mock = server
            .mock("GET", "/builds/test-build-1/logs")
            .match_header("authorization", "Bearer test-token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let result = client.fetch_build_logs("test-token", "test-build-1").await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.finished);
        assert_eq!(response.logs.len(), 2);
        assert_eq!(response.logs[0].log_message, "Starting build...");
    }

    #[tokio::test]
    async fn test_validate_token_success() {
        let mut server = Server::new_async().await;
        let client = GarnixClient::with_base_url(server.url());

        let _mock = server
            .mock("GET", "/user")
            .match_header("authorization", "Bearer valid-token")
            .with_status(200)
            .with_body(r#"{"username": "testuser"}"#)
            .create_async()
            .await;

        let result = client.validate_token("valid-token").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_token_unauthorized() {
        let mut server = Server::new_async().await;
        let client = GarnixClient::with_base_url(server.url());

        let _mock = server
            .mock("GET", "/user")
            .match_header("authorization", "Bearer invalid-token")
            .with_status(401)
            .create_async()
            .await;

        let result = client.validate_token("invalid-token").await;
        assert!(matches!(result, Err(GarnixError::AuthenticationError(_))));
    }

    #[test]
    fn test_client_creation() {
        let client = GarnixClient::new();
        assert_eq!(client.base_url(), "https://garnix.io/api");

        let client = GarnixClient::with_base_url("https://custom.api.url");
        assert_eq!(client.base_url(), "https://custom.api.url");

        let custom_client = reqwest::Client::new();
        let client = GarnixClient::with_client(custom_client);
        assert_eq!(client.base_url(), "https://garnix.io/api");

        let custom_client = reqwest::Client::new();
        let client = GarnixClient::with_client_and_url(custom_client, "https://custom.url");
        assert_eq!(client.base_url(), "https://custom.url");
    }
}
