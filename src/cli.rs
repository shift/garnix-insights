//! Command-line interface for Garnix Insights

use crate::client::GarnixClient;
use crate::error::{GarnixError, GarnixResult};
use crate::mcp::{negotiate_version, GarnixMcpServer};
use crate::server::GarnixHttpServer;
use clap::{Parser, Subcommand};
use tracing::{error, info};

/// Garnix Insights - Fetch CI build status from Garnix.io
#[derive(Parser, Debug)]
#[command(name = "garnix-insights")]
#[command(about = "Fetch CI build status from Garnix.io")]
#[command(version)]
pub struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// JWT authentication token for Garnix.io API (can also be set via GARNIX_JWT_TOKEN env var)
    #[arg(long, env = "GARNIX_JWT_TOKEN")]
    pub jwt_token: Option<String>,

    /// Git commit ID to fetch build status for (required for fetch command)
    #[arg(long)]
    pub commit_id: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format for results
    #[arg(long, default_value = "human")]
    pub format: OutputFormat,

    /// MCP protocol version: latest|stable|legacy|YYYY-MM-DD
    #[arg(long, env = "GARNIX_MCP_PROTOCOL_VERSION")]
    pub mcp_version: Option<String>,
}

/// Available output formats
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum OutputFormat {
    /// Human-readable output with colors and emojis
    Human,
    /// JSON output
    Json,
    /// Plain text output
    Plain,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch build status for a specific commit (default command)
    Fetch {
        /// JWT authentication token
        #[arg(long, env = "GARNIX_JWT_TOKEN")]
        jwt_token: Option<String>,
        /// Git commit ID
        #[arg(long)]
        commit_id: String,
    },
    /// Start HTTP server mode
    Server {
        /// Address to bind the server to
        #[arg(long, default_value = "127.0.0.1")]
        bind_address: String,
        /// Port to bind the server to
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Start MCP (Model Context Protocol) server mode
    Mcp,
    /// Validate a JWT token
    ValidateToken {
        /// JWT authentication token to validate
        #[arg(long, env = "GARNIX_JWT_TOKEN")]
        jwt_token: String,
    },
    /// Get build logs for a specific build
    Logs {
        /// JWT authentication token
        #[arg(long, env = "GARNIX_JWT_TOKEN")]
        jwt_token: String,
        /// Build ID to fetch logs for
        #[arg(long)]
        build_id: String,
    },
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Run the CLI application
    pub async fn run(self) -> GarnixResult<()> {
        // Initialize tracing
        let level = if self.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        };

        tracing_subscriber::fmt().with_max_level(level).init();

        info!("Starting Garnix Insights v{}", env!("CARGO_PKG_VERSION"));

        let client = GarnixClient::new();

        match &self.command {
            Some(Commands::Fetch {
                jwt_token,
                commit_id,
            }) => {
                let token = jwt_token
                    .as_ref()
                    .or(self.jwt_token.as_ref())
                    .ok_or_else(|| GarnixError::ConfigError("JWT token is required".to_string()))?;

                self.fetch_build_status(&client, token, commit_id).await
            }
            Some(Commands::Server { bind_address, port }) => {
                info!("Starting HTTP server on {}:{}", bind_address, port);
                let server = GarnixHttpServer::with_client(client)
                    .bind_address(bind_address.clone())
                    .port(*port);
                server.run().await
            }
            Some(Commands::Mcp) => {
                info!("Starting MCP server");
                let requested = self.mcp_version.as_deref();
                let version = negotiate_version(requested);
                info!("MCP protocol version: {}", version.as_str());
                let server = GarnixMcpServer::with_client_and_version(client, version);
                server.run_stdio().await
            }
            Some(Commands::ValidateToken { jwt_token }) => {
                self.validate_token(&client, jwt_token).await
            }
            Some(Commands::Logs {
                jwt_token,
                build_id,
            }) => self.fetch_build_logs(&client, jwt_token, build_id).await,
            None => {
                // Default behavior - try to fetch build status if we have the required args
                match (&self.jwt_token, &self.commit_id) {
                    (Some(token), Some(commit_id)) => {
                        self.fetch_build_status(&client, token, commit_id).await
                    }
                    _ => {
                        error!("No command specified and missing required arguments");
                        Err(GarnixError::ConfigError(
                            "Either specify a subcommand or provide --jwt-token and --commit-id"
                                .to_string(),
                        ))
                    }
                }
            }
        }
    }

    /// Fetch and display build status
    async fn fetch_build_status(
        &self,
        client: &GarnixClient,
        jwt_token: &str,
        commit_id: &str,
    ) -> GarnixResult<()> {
        info!("Fetching build status for commit: {}", commit_id);

        let response = client.fetch_build_status(jwt_token, commit_id).await?;

        match self.format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&response)?);
            }
            OutputFormat::Human => {
                self.print_human_readable(&response);
            }
            OutputFormat::Plain => {
                self.print_plain_text(&response);
            }
        }

        Ok(())
    }

    /// Validate JWT token
    async fn validate_token(&self, client: &GarnixClient, jwt_token: &str) -> GarnixResult<()> {
        info!("Validating JWT token");

        match client.validate_token(jwt_token).await {
            Ok(_) => {
                match self.format {
                    OutputFormat::Json => {
                        println!(r#"{{"valid": true, "message": "Token is valid"}}"#);
                    }
                    _ => {
                        println!("[OK] JWT token is valid");
                    }
                }
                Ok(())
            }
            Err(e) => {
                match self.format {
                    OutputFormat::Json => {
                        println!(r#"{{"valid": false, "error": "{}"}}"#, e);
                    }
                    _ => {
                        println!("[FAIL] JWT token is invalid: {}", e);
                    }
                }
                Err(e)
            }
        }
    }

    /// Fetch and display build logs
    async fn fetch_build_logs(
        &self,
        client: &GarnixClient,
        jwt_token: &str,
        build_id: &str,
    ) -> GarnixResult<()> {
        info!("Fetching build logs for build: {}", build_id);

        let response = client.fetch_build_logs(jwt_token, build_id).await?;

        match self.format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&response)?);
            }
            OutputFormat::Human | OutputFormat::Plain => {
                if response.logs.is_empty() {
                    println!("No logs available for build {}", build_id);
                } else {
                    println!(
                        "Logs for build {} (finished: {}):",
                        build_id, response.finished
                    );
                    println!("{}", "=".repeat(60));

                    for log_entry in &response.logs {
                        println!("[{}] {}", log_entry.timestamp, log_entry.log_message);
                    }
                }
            }
        }

        Ok(())
    }

    /// Print build status in human-readable format
    fn print_human_readable(&self, response: &crate::types::GarnixResponse) {
        use crate::format_build_summary;

        println!("{}", format_build_summary(response));

        // Print individual builds if there are any failures
        if response.summary.failed > 0 {
            println!("\n[SEARCH] Failed Builds:");
            for build in response.failed_builds() {
                println!(
                    "  • {} ({}): {}",
                    build.package,
                    build.system.as_deref().unwrap_or("unknown"),
                    build.status_with_emoji()
                );
                if let Some(drv_path) = &build.drv_path {
                    println!("    Derivation: {}", drv_path);
                }
            }
        }

        // Print success rate
        let success_rate = response.success_rate();
        let emoji = if success_rate == 100.0 {
            "[SUCCESS]"
        } else if success_rate >= 80.0 {
            "[GOOD]"
        } else {
            "[WARNING]"
        };
        println!("\n{} Success Rate: {:.1}%", emoji, success_rate);
    }

    /// Print build status in plain text format
    fn print_plain_text(&self, response: &crate::types::GarnixResponse) {
        println!("Build Status for {}", response.summary.git_commit);
        println!(
            "Repository: {}/{}",
            response.summary.repo_owner, response.summary.repo_name
        );
        println!("Branch: {}", response.summary.branch);
        println!("Started: {}", response.summary.start_time);
        println!();

        println!("Summary:");
        println!("  Succeeded: {}", response.summary.succeeded);
        println!("  Failed: {}", response.summary.failed);
        println!("  Pending: {}", response.summary.pending);
        println!("  Cancelled: {}", response.summary.cancelled);
        println!();

        if !response.builds.is_empty() {
            println!("Individual Builds:");
            for build in &response.builds {
                println!(
                    "  {} - {} ({})",
                    build.package,
                    build.status,
                    build.system.as_deref().unwrap_or("unknown")
                );
            }
        }

        println!("Success Rate: {:.1}%", response.success_rate());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Build, GarnixResponse, Summary};

    fn create_test_response() -> GarnixResponse {
        GarnixResponse {
            summary: Summary {
                repo_owner: "testuser".to_string(),
                repo_name: "testrepo".to_string(),
                repo_is_public: true,
                git_commit: "7a2f5e9c1b4d8a3e6f2a9e5c8b1d4f7a3c6e9b2d".to_string(),
                branch: "main".to_string(),
                req_user: "testuser".to_string(),
                start_time: "2024-01-01T00:00:00Z".to_string(),
                succeeded: 2,
                failed: 1,
                pending: 0,
                cancelled: 0,
            },
            builds: vec![
                Build {
                    id: "build1".to_string(),
                    repo_user: "testuser".to_string(),
                    repo_name: "testrepo".to_string(),
                    branch: "main".to_string(),
                    repo_is_public: true,
                    git_commit: "7a2f5e9c1b4d8a3e6f2a9e5c8b1d4f7a3c6e9b2d".to_string(),
                    package: "package1".to_string(),
                    package_type: "derivation".to_string(),
                    system: Some("x86_64-linux".to_string()),
                    req_user: "testuser".to_string(),
                    status: "Success".to_string(),
                    start_time: "2024-01-01T00:00:00Z".to_string(),
                    end_time: "2024-01-01T00:01:00Z".to_string(),
                    drv_path: None,
                    output_paths: None,
                    github_run_id: 123,
                    wants_incrementalism: false,
                    eval_host: "eval.garnix.io".to_string(),
                    uploaded_to_cache: true,
                },
                Build {
                    id: "build2".to_string(),
                    repo_user: "testuser".to_string(),
                    repo_name: "testrepo".to_string(),
                    branch: "main".to_string(),
                    repo_is_public: true,
                    git_commit: "7a2f5e9c1b4d8a3e6f2a9e5c8b1d4f7a3c6e9b2d".to_string(),
                    package: "package2".to_string(),
                    package_type: "derivation".to_string(),
                    system: Some("x86_64-linux".to_string()),
                    req_user: "testuser".to_string(),
                    status: "Failed".to_string(),
                    start_time: "2024-01-01T00:00:00Z".to_string(),
                    end_time: "2024-01-01T00:01:30Z".to_string(),
                    drv_path: Some("/nix/store/test.drv".to_string()),
                    output_paths: None,
                    github_run_id: 124,
                    wants_incrementalism: false,
                    eval_host: "eval.garnix.io".to_string(),
                    uploaded_to_cache: false,
                },
            ],
            runs: vec![],
        }
    }

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::try_parse_from(&[
            "garnix-insights",
            "fetch",
            "--jwt-token",
            "test-token",
            "--commit-id",
            "5d9e2f7a1c4b8e3a6f1d9e2a7f5c8b3e6a1f4d9e",
        ])
        .unwrap();

        match cli.command.unwrap() {
            Commands::Fetch {
                jwt_token,
                commit_id,
            } => {
                assert_eq!(jwt_token.unwrap(), "test-token");
                assert_eq!(commit_id, "5d9e2f7a1c4b8e3a6f1d9e2a7f5c8b3e6a1f4d9e");
            }
            _ => panic!("Wrong command parsed"),
        }
    }

    #[test]
    fn test_cli_server_parsing() {
        let cli = Cli::try_parse_from(&[
            "garnix-insights",
            "server",
            "--bind-address",
            "0.0.0.0",
            "--port",
            "3000",
        ])
        .unwrap();

        match cli.command.unwrap() {
            Commands::Server { bind_address, port } => {
                assert_eq!(bind_address, "0.0.0.0");
                assert_eq!(port, 3000);
            }
            _ => panic!("Wrong command parsed"),
        }
    }

    #[test]
    fn test_cli_mcp_parsing() {
        let cli = Cli::try_parse_from(&["garnix-insights", "mcp"]).unwrap();

        assert!(matches!(cli.command.unwrap(), Commands::Mcp));
    }

    #[test]
    fn test_cli_validate_token_parsing() {
        let cli = Cli::try_parse_from(&[
            "garnix-insights",
            "validate-token",
            "--jwt-token",
            "test-token",
        ])
        .unwrap();

        match cli.command.unwrap() {
            Commands::ValidateToken { jwt_token } => {
                assert_eq!(jwt_token, "test-token");
            }
            _ => panic!("Wrong command parsed"),
        }
    }

    #[test]
    fn test_cli_logs_parsing() {
        let cli = Cli::try_parse_from(&[
            "garnix-insights",
            "logs",
            "--jwt-token",
            "test-token",
            "--build-id",
            "build123",
        ])
        .unwrap();

        match cli.command.unwrap() {
            Commands::Logs {
                jwt_token,
                build_id,
            } => {
                assert_eq!(jwt_token, "test-token");
                assert_eq!(build_id, "build123");
            }
            _ => panic!("Wrong command parsed"),
        }
    }

    #[test]
    fn test_output_format_parsing() {
        let cli = Cli::try_parse_from(&[
            "garnix-insights",
            "--format",
            "json",
            "fetch",
            "--jwt-token",
            "test",
            "--commit-id",
            "6e1f5a8c3d9b2e7a4f8c1e6a9d5b2f7a4c8e1b6d",
        ])
        .unwrap();

        assert!(matches!(cli.format, OutputFormat::Json));
    }

    #[test]
    fn test_verbose_flag() {
        let cli = Cli::try_parse_from(&["garnix-insights", "--verbose", "mcp"]).unwrap();

        assert!(cli.verbose);
    }

    #[test]
    fn test_print_human_readable() {
        let cli = Cli::try_parse_from(&["garnix-insights", "mcp"]).unwrap();
        let response = create_test_response();

        // This test mainly ensures the method doesn't panic
        // In a real test environment, you might capture stdout to verify output
        cli.print_human_readable(&response);
    }

    #[test]
    fn test_print_plain_text() {
        let cli = Cli::try_parse_from(&["garnix-insights", "mcp"]).unwrap();
        let response = create_test_response();

        // This test mainly ensures the method doesn't panic
        cli.print_plain_text(&response);
    }

    #[test]
    fn test_json_format_serialization() {
        let response = create_test_response();

        // Test that the response can be serialised to JSON without error
        let json_result = serde_json::to_string_pretty(&response);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        assert!(json_string.contains("7a2f5e9c1b4d8a3e6f2a9e5c8b1d4f7a3c6e9b2d")); // commit ID
        assert!(json_string.contains("testuser")); // repo owner
    }

    #[test]
    fn test_cli_structure() {
        let cli = Cli {
            jwt_token: Some("token".to_string()),
            commit_id: Some("8b3e6f1a4c9d2e7a5f8b1e4a7c2f5e9a6b3d8f1a".to_string()),
            verbose: true,
            format: OutputFormat::Json,
            mcp_version: None,
            command: Some(Commands::Mcp),
        };

        assert!(cli.verbose);
        assert!(matches!(cli.format, OutputFormat::Json));
        assert_eq!(cli.jwt_token, Some("token".to_string()));
        assert_eq!(
            cli.commit_id,
            Some("8b3e6f1a4c9d2e7a5f8b1e4a7c2f5e9a6b3d8f1a".to_string())
        );
    }

    #[tokio::test]
    async fn test_cli_error_handling() {
        let cli = Cli {
            jwt_token: None,
            commit_id: Some("test123".to_string()),
            verbose: false,
            format: OutputFormat::Human,
            mcp_version: None,
            command: Some(Commands::Fetch {
                jwt_token: None,
                commit_id: "test123".to_string(),
            }),
        };

        // This should fail due to missing JWT token
        let result = cli.run().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_output_format_variants() {
        // Test that all variants can be created
        let _human = OutputFormat::Human;
        let _json = OutputFormat::Json;
        let _plain = OutputFormat::Plain;

        // Test Debug trait
        assert!(!format!("{:?}", OutputFormat::Human).is_empty());
        assert!(!format!("{:?}", OutputFormat::Json).is_empty());
        assert!(!format!("{:?}", OutputFormat::Plain).is_empty());
    }
}
