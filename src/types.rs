//! Type definitions for Garnix API responses and requests

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request parameter for fetching Garnix build status
#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema, Clone)]
pub struct GarnixRequest {
    /// JWT authentication token for Garnix.io API access
    pub jwt_token: String,
    /// Git commit ID (SHA) to fetch build status for
    pub commit_id: String,
}

/// Summary information for a build
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Summary {
    /// Repository owner/organization
    pub repo_owner: String,
    /// Repository name
    pub repo_name: String,
    /// Whether the repository is public
    pub repo_is_public: bool,
    /// Git commit hash
    pub git_commit: String,
    /// Git branch name
    pub branch: String,
    /// User who requested the build
    pub req_user: String,
    /// Build start time
    pub start_time: String,
    /// Number of successful builds
    pub succeeded: u32,
    /// Number of failed builds
    pub failed: u32,
    /// Number of pending builds
    pub pending: u32,
    /// Number of cancelled builds
    pub cancelled: u32,
}

/// Individual build information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Build {
    /// Unique build ID
    pub id: String,
    /// Repository owner/user
    pub repo_user: String,
    /// Repository name
    pub repo_name: String,
    /// Git branch name
    pub branch: String,
    /// Whether the repository is public
    pub repo_is_public: bool,
    /// Git commit hash
    pub git_commit: String,
    /// Package/derivation name
    pub package: String,
    /// Type of package being built
    pub package_type: String,
    /// Target system (e.g., "x86_64-linux")
    pub system: Option<String>,
    /// User who requested the build
    pub req_user: String,
    /// Build status ("Success", "Failed", "Pending", etc.)
    pub status: String,
    /// Build start time
    pub start_time: String,
    /// Build end time
    pub end_time: String,
    /// Nix derivation path
    pub drv_path: Option<String>,
    /// Map of output names to store paths
    pub output_paths: Option<HashMap<String, String>>,
    /// GitHub Actions run ID
    pub github_run_id: u64,
    /// Whether incremental builds are wanted
    pub wants_incrementalism: bool,
    /// Evaluation host
    pub eval_host: String,
    /// Whether build artifacts were uploaded to cache
    pub uploaded_to_cache: bool,
}

/// Log entry from build logs
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogEntry {
    /// Timestamp of the log entry
    pub timestamp: String,
    /// Log message content
    pub log_message: String,
}

/// Response from the logs API endpoint
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogResponse {
    /// Whether the build has finished
    pub finished: bool,
    /// List of log entries
    pub logs: Vec<LogEntry>,
}

/// Main response structure from Garnix API
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GarnixResponse {
    /// Build summary information
    pub summary: Summary,
    /// List of individual builds
    pub builds: Vec<Build>,
    /// Additional run information (raw JSON values)
    pub runs: Vec<serde_json::Value>,
}

/// Request structure for the HTTP API endpoint
#[derive(Debug, Deserialize, Clone)]
pub struct BuildStatusRequest {
    /// JWT authentication token
    pub jwt_token: String,
    /// Git commit ID
    pub commit_id: String,
}

/// Build status enumeration for type safety
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildStatus {
    /// Build completed successfully
    Success,
    /// Build failed
    Failed,
    /// Build is still running
    Pending,
    /// Build was cancelled
    Cancelled,
    /// Unknown or other status
    Other(String),
}

impl From<&str> for BuildStatus {
    fn from(status: &str) -> Self {
        match status {
            "Success" => Self::Success,
            "Failed" => Self::Failed,
            "Pending" => Self::Pending,
            "Cancelled" => Self::Cancelled,
            other => Self::Other(other.to_string()),
        }
    }
}

impl std::fmt::Display for BuildStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildStatus::Success => write!(f, "Success"),
            BuildStatus::Failed => write!(f, "Failed"),
            BuildStatus::Pending => write!(f, "Pending"),
            BuildStatus::Cancelled => write!(f, "Cancelled"),
            BuildStatus::Other(status) => write!(f, "{}", status),
        }
    }
}

impl Build {
    /// Get the build status as a typed enum
    pub fn status_enum(&self) -> BuildStatus {
        BuildStatus::from(self.status.as_str())
    }

    /// Check if the build was successful
    pub fn is_successful(&self) -> bool {
        matches!(self.status_enum(), BuildStatus::Success)
    }

    /// Check if the build failed
    pub fn is_failed(&self) -> bool {
        matches!(self.status_enum(), BuildStatus::Failed)
    }

    /// Check if the build is still running
    pub fn is_pending(&self) -> bool {
        matches!(self.status_enum(), BuildStatus::Pending)
    }

    /// Get a human-readable status with emoji
    pub fn status_with_emoji(&self) -> String {
        let emoji = match self.status_enum() {
            BuildStatus::Success => "✅",
            BuildStatus::Failed => "❌",
            BuildStatus::Pending => "⏳",
            BuildStatus::Cancelled => "🚫",
            BuildStatus::Other(_) => "❓",
        };
        format!("{} {}", emoji, self.status)
    }
}

impl GarnixResponse {
    /// Get all failed builds
    pub fn failed_builds(&self) -> Vec<&Build> {
        self.builds
            .iter()
            .filter(|build| build.is_failed())
            .collect()
    }

    /// Get all successful builds
    pub fn successful_builds(&self) -> Vec<&Build> {
        self.builds
            .iter()
            .filter(|build| build.is_successful())
            .collect()
    }

    /// Get all pending builds
    pub fn pending_builds(&self) -> Vec<&Build> {
        self.builds
            .iter()
            .filter(|build| build.is_pending())
            .collect()
    }

    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.builds.is_empty() {
            return 100.0;
        }
        (self.summary.succeeded as f64 / self.builds.len() as f64) * 100.0
    }

    /// Check if all builds were successful
    pub fn all_successful(&self) -> bool {
        self.summary.failed == 0 && self.summary.cancelled == 0 && self.summary.pending == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_status_from_str() {
        assert_eq!(BuildStatus::from("Success"), BuildStatus::Success);
        assert_eq!(BuildStatus::from("Failed"), BuildStatus::Failed);
        assert_eq!(BuildStatus::from("Pending"), BuildStatus::Pending);
        assert_eq!(BuildStatus::from("Cancelled"), BuildStatus::Cancelled);
        assert_eq!(
            BuildStatus::from("Unknown"),
            BuildStatus::Other("Unknown".to_string())
        );
    }

    #[test]
    fn test_build_status_display() {
        assert_eq!(BuildStatus::Success.to_string(), "Success");
        assert_eq!(BuildStatus::Failed.to_string(), "Failed");
        assert_eq!(
            BuildStatus::Other("Custom".to_string()).to_string(),
            "Custom"
        );
    }

    #[test]
    fn test_build_status_methods() {
        let successful_build = Build {
            id: "1".to_string(),
            repo_user: "test".to_string(),
            repo_name: "test".to_string(),
            branch: "main".to_string(),
            repo_is_public: true,
            git_commit: "abc123".to_string(),
            package: "test-pkg".to_string(),
            package_type: "derivation".to_string(),
            system: Some("x86_64-linux".to_string()),
            req_user: "user".to_string(),
            status: "Success".to_string(),
            start_time: "2024-01-01T00:00:00Z".to_string(),
            end_time: "2024-01-01T00:01:00Z".to_string(),
            drv_path: None,
            output_paths: None,
            github_run_id: 123,
            wants_incrementalism: false,
            eval_host: "host".to_string(),
            uploaded_to_cache: true,
        };

        assert!(successful_build.is_successful());
        assert!(!successful_build.is_failed());
        assert!(!successful_build.is_pending());
        assert_eq!(successful_build.status_with_emoji(), "✅ Success");
    }

    #[test]
    fn test_garnix_response_methods() {
        let response = GarnixResponse {
            summary: Summary {
                repo_owner: "test".to_string(),
                repo_name: "test".to_string(),
                repo_is_public: true,
                git_commit: "abc123".to_string(),
                branch: "main".to_string(),
                req_user: "user".to_string(),
                start_time: "2024-01-01T00:00:00Z".to_string(),
                succeeded: 1,
                failed: 1,
                pending: 0,
                cancelled: 0,
            },
            builds: vec![
                Build {
                    id: "1".to_string(),
                    repo_user: "test".to_string(),
                    repo_name: "test".to_string(),
                    branch: "main".to_string(),
                    repo_is_public: true,
                    git_commit: "abc123".to_string(),
                    package: "pkg1".to_string(),
                    package_type: "derivation".to_string(),
                    system: Some("x86_64-linux".to_string()),
                    req_user: "user".to_string(),
                    status: "Success".to_string(),
                    start_time: "2024-01-01T00:00:00Z".to_string(),
                    end_time: "2024-01-01T00:01:00Z".to_string(),
                    drv_path: None,
                    output_paths: None,
                    github_run_id: 123,
                    wants_incrementalism: false,
                    eval_host: "host".to_string(),
                    uploaded_to_cache: true,
                },
                Build {
                    id: "2".to_string(),
                    repo_user: "test".to_string(),
                    repo_name: "test".to_string(),
                    branch: "main".to_string(),
                    repo_is_public: true,
                    git_commit: "abc123".to_string(),
                    package: "pkg2".to_string(),
                    package_type: "derivation".to_string(),
                    system: Some("x86_64-linux".to_string()),
                    req_user: "user".to_string(),
                    status: "Failed".to_string(),
                    start_time: "2024-01-01T00:00:00Z".to_string(),
                    end_time: "2024-01-01T00:01:00Z".to_string(),
                    drv_path: None,
                    output_paths: None,
                    github_run_id: 124,
                    wants_incrementalism: false,
                    eval_host: "host".to_string(),
                    uploaded_to_cache: false,
                },
            ],
            runs: vec![],
        };

        assert_eq!(response.failed_builds().len(), 1);
        assert_eq!(response.successful_builds().len(), 1);
        assert_eq!(response.pending_builds().len(), 0);
        assert!((response.success_rate() - 50.0).abs() < 0.01);
        assert!(!response.all_successful());
    }

    #[test]
    fn test_garnix_response_all_successful() {
        let response = GarnixResponse {
            summary: Summary {
                repo_owner: "test".to_string(),
                repo_name: "test".to_string(),
                repo_is_public: true,
                git_commit: "abc123".to_string(),
                branch: "main".to_string(),
                req_user: "user".to_string(),
                start_time: "2024-01-01T00:00:00Z".to_string(),
                succeeded: 1,
                failed: 0,
                pending: 0,
                cancelled: 0,
            },
            builds: vec![],
            runs: vec![],
        };

        assert!(response.all_successful());
        assert!((response.success_rate() - 100.0).abs() < f64::EPSILON);
    }
}
