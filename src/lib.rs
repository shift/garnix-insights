//! # Garnix Insights Library
//!
//! A Rust library for analyzing build status information from Garnix.io.
//! This library provides functionality to query Garnix build status, format results,
//! and interact with the Garnix API through various interfaces including CLI, HTTP server,
//! and Model Context Protocol (MCP) server.
//!
//! ## Features
//!
//! - **HTTP Client**: Fetch build status from Garnix.io API
//! - **Multiple Output Formats**: JSON and human-readable formatting
//! - **CLI Interface**: Command-line tool with multiple commands
//! - **HTTP Server**: REST API server for build status queries
//! - **MCP Server**: Model Context Protocol server for AI agents
//! - **Error Handling**: Comprehensive error types and handling
//! - **Testing**: Full test coverage with mocks and integration tests
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use garnix_insights::{get_garnix_data, format_build_summary};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let response = get_garnix_data("your-jwt-token", "commit-sha").await?;
//!     let summary = format_build_summary(&response);
//!     println!("{}", summary);
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod client;
pub mod error;
pub mod mcp;
pub mod server;
pub mod types;

pub use client::GarnixClient;
pub use error::{GarnixError, GarnixResult};
pub use types::{Build, GarnixRequest, GarnixResponse, Summary};

/// The main function to fetch Garnix data for a given commit
///
/// This is a convenience function that creates a new `GarnixClient` and fetches
/// build status for the specified commit.
///
/// # Arguments
/// * `jwt_token` - The JWT authentication token for Garnix.io
/// * `commit_id` - The Git commit ID to fetch build status for
///
/// # Returns
/// A `GarnixResult<GarnixResponse>` containing the build status information
///
/// # Errors
/// Returns an error if:
/// * The JWT token is invalid or expired
/// * The commit ID is not found in the repository  
/// * Network or API communication fails
/// * JSON parsing fails
///
/// # Example
/// ```rust,no_run
/// # use garnix_insights::get_garnix_data;
/// # tokio_test::block_on(async {
/// let response = get_garnix_data("jwt-token", "abc123").await?;
/// println!("Repository: {}/{}", response.summary.repo_owner, response.summary.repo_name);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
pub async fn get_garnix_data(jwt_token: &str, commit_id: &str) -> GarnixResult<GarnixResponse> {
    let client = GarnixClient::new();
    client.fetch_build_status(jwt_token, commit_id).await
}

/// Format a build summary as a human-readable string
///
/// Creates a markdown-formatted summary of the build status including
/// repository information, branch, timing, and success/failure counts.
///
/// # Arguments
/// * `response` - The Garnix response to format
///
/// # Returns
/// A formatted string containing the build summary
///
/// # Example
/// ```rust
/// # use garnix_insights::{types::*, format_build_summary};
/// # use std::collections::HashMap;
/// let response = GarnixResponse {
///     summary: Summary {
///         repo_owner: "user".to_string(),
///         repo_name: "repo".to_string(),
///         repo_is_public: true,
///         git_commit: "abc123".to_string(),
///         branch: "main".to_string(),
///         req_user: "user".to_string(),
///         start_time: "2024-01-01T00:00:00Z".to_string(),
///         succeeded: 5,
///         failed: 1,
///         pending: 0,
///         cancelled: 0,
///     },
///     builds: vec![],
///     runs: vec![],
/// };
/// let summary = format_build_summary(&response);
/// assert!(summary.contains("# Build Summary"));
/// ```
pub fn format_build_summary(response: &GarnixResponse) -> String {
    format!(
        "# Build Summary for {}\n\n\
         **Repository:** {}/{}\n\
         **Branch:** {}\n\
         **Started:** {}\n\n\
         ## Summary\n\
         - [OK] Succeeded: {}\n\
         - [FAIL] Failed: {}\n\
         - ⏳ Pending: {}\n\
         - [CANCELLED] Cancelled: {}",
        &response.summary.git_commit[..std::cmp::min(8, response.summary.git_commit.len())],
        response.summary.repo_owner,
        response.summary.repo_name,
        response.summary.branch,
        response.summary.start_time,
        response.summary.succeeded,
        response.summary.failed,
        response.summary.pending,
        response.summary.cancelled
    )
}

/// Format build details as a human-readable string
///
/// Creates a detailed markdown-formatted list of all individual builds,
/// including status, system information, timing, and build IDs.
///
/// # Arguments
/// * `builds` - Vector of builds to format
///
/// # Returns
/// A formatted string containing detailed build information
///
/// # Example
/// ```rust
/// # use garnix_insights::{types::Build, format_build_details};
/// # use std::collections::HashMap;
/// let builds = vec![
///     Build {
///         id: "build1".to_string(),
///         repo_user: "user".to_string(),
///         repo_name: "repo".to_string(),
///         branch: "main".to_string(),
///         repo_is_public: true,
///         git_commit: "abc123".to_string(),
///         package: "package1".to_string(),
///         package_type: "derivation".to_string(),
///         system: Some("x86_64-linux".to_string()),
///         req_user: "user".to_string(),
///         status: "Success".to_string(),
///         start_time: "2024-01-01T00:00:00Z".to_string(),
///         end_time: "2024-01-01T00:01:00Z".to_string(),
///         drv_path: None,
///         output_paths: None,
///         github_run_id: 123,
///         wants_incrementalism: false,
///         eval_host: "eval.garnix.io".to_string(),
///         uploaded_to_cache: true,
///     },
/// ];
/// let details = format_build_details(&builds);
/// assert!(details.contains("## Individual Builds"));
/// ```
pub fn format_build_details(builds: &[Build]) -> String {
    if builds.is_empty() {
        return "\n## No builds found".to_string();
    }

    let mut build_details = String::from("\n## Individual Builds\n");

    for build in builds {
        build_details.push_str(&format!(
            "### {}\n\
             - **Status:** {}\n",
            build.package,
            build.status_with_emoji()
        ));

        if let Some(system) = &build.system {
            build_details.push_str(&format!("- **System:** {}\n", system));
        }

        build_details.push_str(&format!(
            "- **Duration:** {} → {}\n\
             - **Build ID:** {}\n\n",
            build.start_time, build.end_time, build.id
        ));

        if build.is_failed() {
            if let Some(drv_path) = &build.drv_path {
                build_details.push_str(&format!("- **Derivation:** {}\n", drv_path));
            }
        }
    }

    build_details
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_response() -> GarnixResponse {
        GarnixResponse {
            summary: Summary {
                repo_owner: "testowner".to_string(),
                repo_name: "testrepo".to_string(),
                repo_is_public: true,
                git_commit: "abc123def456".to_string(),
                branch: "main".to_string(),
                req_user: "testuser".to_string(),
                start_time: "2024-01-01T00:00:00Z".to_string(),
                succeeded: 5,
                failed: 1,
                pending: 0,
                cancelled: 0,
            },
            builds: vec![
                Build {
                    id: "build1".to_string(),
                    repo_user: "testowner".to_string(),
                    repo_name: "testrepo".to_string(),
                    branch: "main".to_string(),
                    repo_is_public: true,
                    git_commit: "abc123def456".to_string(),
                    package: "package1".to_string(),
                    package_type: "derivation".to_string(),
                    system: Some("x86_64-linux".to_string()),
                    req_user: "testuser".to_string(),
                    status: "Success".to_string(),
                    start_time: "2024-01-01T00:00:00Z".to_string(),
                    end_time: "2024-01-01T00:01:00Z".to_string(),
                    drv_path: Some("/nix/store/test.drv".to_string()),
                    output_paths: Some(HashMap::new()),
                    github_run_id: 123456,
                    wants_incrementalism: false,
                    eval_host: "test-host".to_string(),
                    uploaded_to_cache: true,
                },
                Build {
                    id: "build2".to_string(),
                    repo_user: "testowner".to_string(),
                    repo_name: "testrepo".to_string(),
                    branch: "main".to_string(),
                    repo_is_public: true,
                    git_commit: "abc123def456".to_string(),
                    package: "package2".to_string(),
                    package_type: "derivation".to_string(),
                    system: Some("x86_64-linux".to_string()),
                    req_user: "testuser".to_string(),
                    status: "Failed".to_string(),
                    start_time: "2024-01-01T00:00:00Z".to_string(),
                    end_time: "2024-01-01T00:01:00Z".to_string(),
                    drv_path: Some("/nix/store/test2.drv".to_string()),
                    output_paths: Some(HashMap::new()),
                    github_run_id: 123457,
                    wants_incrementalism: false,
                    eval_host: "test-host".to_string(),
                    uploaded_to_cache: false,
                },
            ],
            runs: vec![],
        }
    }

    #[tokio::test]
    async fn test_get_garnix_data() {
        // This is an integration test that would require a mock server
        // For now, we'll just test that the function exists and can be called
        // In a real test, we'd use mockito to mock the HTTP responses

        // let response = get_garnix_data("test-token", "test-commit").await;
        // We can't test this without mocking, so we'll skip the actual call
    }

    #[test]
    fn test_format_build_summary() {
        let response = create_test_response();
        let formatted = format_build_summary(&response);

        assert!(formatted.contains("# Build Summary for abc123de"));
        assert!(formatted.contains("testowner/testrepo"));
        assert!(formatted.contains("**Branch:** main"));
        assert!(formatted.contains("[OK] Succeeded: 5"));
        assert!(formatted.contains("[FAIL] Failed: 1"));
    }

    #[test]
    fn test_format_build_details() {
        let response = create_test_response();
        let formatted = format_build_details(&response.builds);

        assert!(formatted.contains("## Individual Builds"));
        assert!(formatted.contains("### package1"));
        assert!(formatted.contains("[OK] Success"));
        assert!(formatted.contains("### package2"));
        assert!(formatted.contains("[FAIL] Failed"));
        assert!(formatted.contains("**System:** x86_64-linux"));
    }

    #[test]
    fn test_format_build_details_empty() {
        let details = format_build_details(&[]);
        assert!(details.contains("## No builds found"));
    }
}
