//! Error types for the Garnix Fetcher library

use std::fmt;

/// Main error type for Garnix operations
#[derive(Debug, Clone)]
pub enum GarnixError {
    /// Network-related errors (connection issues, timeouts, etc.)
    NetworkError(String),
    /// Authentication failures (invalid JWT tokens, etc.)
    AuthenticationError(String),
    /// Resource not found errors (commit, build, etc.)
    NotFound(String),
    /// Rate limiting errors
    RateLimit(String),
    /// General API errors from Garnix
    ApiError(String),
    /// JSON parsing errors
    ParseError(String),
    /// Configuration errors
    ConfigError(String),
    /// I/O errors (file operations, etc.)
    IoError(String),
    /// Validation errors
    ValidationError(String),
}

impl fmt::Display for GarnixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GarnixError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            GarnixError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            GarnixError::NotFound(msg) => write!(f, "Not found: {}", msg),
            GarnixError::RateLimit(msg) => write!(f, "Rate limit exceeded: {}", msg),
            GarnixError::ApiError(msg) => write!(f, "API error: {}", msg),
            GarnixError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            GarnixError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            GarnixError::IoError(msg) => write!(f, "I/O error: {}", msg),
            GarnixError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for GarnixError {}

impl From<reqwest::Error> for GarnixError {
    fn from(error: reqwest::Error) -> Self {
        GarnixError::NetworkError(error.to_string())
    }
}

impl From<serde_json::Error> for GarnixError {
    fn from(error: serde_json::Error) -> Self {
        GarnixError::ParseError(error.to_string())
    }
}

impl From<std::io::Error> for GarnixError {
    fn from(error: std::io::Error) -> Self {
        GarnixError::IoError(error.to_string())
    }
}

impl From<anyhow::Error> for GarnixError {
    fn from(error: anyhow::Error) -> Self {
        GarnixError::ApiError(error.to_string())
    }
}

impl GarnixError {
    /// Create a new network error
    pub fn network<T: Into<String>>(message: T) -> Self {
        GarnixError::NetworkError(message.into())
    }

    /// Create a new authentication error
    pub fn auth<T: Into<String>>(message: T) -> Self {
        GarnixError::AuthenticationError(message.into())
    }

    /// Create a new not found error
    pub fn not_found<T: Into<String>>(message: T) -> Self {
        GarnixError::NotFound(message.into())
    }

    /// Create a new rate limit error
    pub fn rate_limit<T: Into<String>>(message: T) -> Self {
        GarnixError::RateLimit(message.into())
    }

    /// Create a new API error
    pub fn api<T: Into<String>>(message: T) -> Self {
        GarnixError::ApiError(message.into())
    }

    /// Create a new parse error
    pub fn parse<T: Into<String>>(message: T) -> Self {
        GarnixError::ParseError(message.into())
    }

    /// Create a new configuration error
    pub fn config<T: Into<String>>(message: T) -> Self {
        GarnixError::ConfigError(message.into())
    }

    /// Create a new I/O error
    pub fn io<T: Into<String>>(message: T) -> Self {
        GarnixError::IoError(message.into())
    }

    /// Create a new validation error
    pub fn validation<T: Into<String>>(message: T) -> Self {
        GarnixError::ValidationError(message.into())
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            GarnixError::NetworkError(_) => true,
            GarnixError::RateLimit(_) => true,
            GarnixError::ApiError(msg) => {
                // Some API errors might be retryable (5xx status codes)
                msg.contains("HTTP 5")
            },
            _ => false,
        }
    }

    /// Check if this is a client error (4xx-like errors that shouldn't be retried)
    pub fn is_client_error(&self) -> bool {
        match self {
            GarnixError::AuthenticationError(_) => true,
            GarnixError::NotFound(_) => true,
            GarnixError::ValidationError(_) => true,
            GarnixError::ConfigError(_) => true,
            GarnixError::ApiError(msg) => {
                // 4xx client errors shouldn't be retried
                msg.contains("HTTP 4")
            },
            _ => false,
        }
    }

    /// Get the error category as a string
    pub fn category(&self) -> &'static str {
        match self {
            GarnixError::NetworkError(_) => "network",
            GarnixError::AuthenticationError(_) => "authentication",
            GarnixError::NotFound(_) => "not_found",
            GarnixError::RateLimit(_) => "rate_limit",
            GarnixError::ApiError(_) => "api",
            GarnixError::ParseError(_) => "parse",
            GarnixError::ConfigError(_) => "config",
            GarnixError::IoError(_) => "io",
            GarnixError::ValidationError(_) => "validation",
        }
    }
}

/// Result type alias for Garnix operations
pub type GarnixResult<T> = Result<T, GarnixError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = GarnixError::NetworkError("Connection refused".to_string());
        assert_eq!(error.to_string(), "Network error: Connection refused");

        let error = GarnixError::AuthenticationError("Invalid token".to_string());
        assert_eq!(error.to_string(), "Authentication error: Invalid token");

        let error = GarnixError::NotFound("Commit abc123 not found".to_string());
        assert_eq!(error.to_string(), "Not found: Commit abc123 not found");
    }

    #[test]
    fn test_error_constructors() {
        let error = GarnixError::network("Test message");
        assert!(matches!(error, GarnixError::NetworkError(_)));
        assert_eq!(error.to_string(), "Network error: Test message");

        let error = GarnixError::auth("Bad token");
        assert!(matches!(error, GarnixError::AuthenticationError(_)));

        let error = GarnixError::not_found("Missing resource");
        assert!(matches!(error, GarnixError::NotFound(_)));
    }

    #[test]
    fn test_error_categorization() {
        let network_error = GarnixError::NetworkError("Connection failed".to_string());
        assert!(network_error.is_retryable());
        assert!(!network_error.is_client_error());
        assert_eq!(network_error.category(), "network");

        let auth_error = GarnixError::AuthenticationError("Invalid token".to_string());
        assert!(!auth_error.is_retryable());
        assert!(auth_error.is_client_error());
        assert_eq!(auth_error.category(), "authentication");

        let rate_limit_error = GarnixError::RateLimit("Too many requests".to_string());
        assert!(rate_limit_error.is_retryable());
        assert!(!rate_limit_error.is_client_error());
        assert_eq!(rate_limit_error.category(), "rate_limit");

        let not_found_error = GarnixError::NotFound("Resource missing".to_string());
        assert!(!not_found_error.is_retryable());
        assert!(not_found_error.is_client_error());
        assert_eq!(not_found_error.category(), "not_found");
    }

    #[test]
    fn test_api_error_retryable() {
        let server_error = GarnixError::ApiError("HTTP 500: Internal Server Error".to_string());
        assert!(server_error.is_retryable());
        assert!(!server_error.is_client_error());

        let client_error = GarnixError::ApiError("HTTP 400: Bad Request".to_string());
        assert!(!client_error.is_retryable());
        assert!(client_error.is_client_error());

        let other_error = GarnixError::ApiError("Unknown API error".to_string());
        assert!(!other_error.is_retryable());
        assert!(!other_error.is_client_error());
    }

    #[test]
    fn test_error_from_conversions() {
        #[test]
    fn test_error_from_conversions() {
        // Test serde_json error conversion
        let json_error = serde_json::from_str::<i32>("not a number").unwrap_err();
        let garnix_error: GarnixError = json_error.into();
        assert!(matches!(garnix_error, GarnixError::ParseError(_)));

        // Test io error conversion
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let garnix_error: GarnixError = io_error.into();
        assert!(matches!(garnix_error, GarnixError::IoError(_)));
    }

        let anyhow_error = anyhow::anyhow!("Something went wrong");
        let garnix_error: GarnixError = anyhow_error.into();
        assert!(matches!(garnix_error, GarnixError::ApiError(_)));
    }

    #[test]
    fn test_error_implements_std_error() {
        let error = GarnixError::NetworkError("Test error".to_string());
        let _: &dyn std::error::Error = &error;
    }
}
