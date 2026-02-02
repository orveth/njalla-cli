//! Error types for njalla-cli.

use thiserror::Error;

/// All errors that can occur in njalla-cli.
#[derive(Error, Debug)]
pub enum NjallaError {
    /// API token not found.
    #[error("No API token found. Set NJALLA_API_TOKEN or add api_token to ./config.toml")]
    MissingToken,

    /// HTTP request failed.
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("API error: {message}")]
    Api {
        /// Error message from the API.
        message: String,
    },

    /// Domain is not available for registration.
    #[error("Domain not available: {0}")]
    DomainNotAvailable(String),

    /// Registration timed out waiting for completion.
    #[error("Registration timeout for {domain} after {timeout_secs}s")]
    RegistrationTimeout {
        /// Domain being registered.
        domain: String,
        /// Timeout in seconds.
        timeout_secs: u64,
    },

    /// JSON parsing failed.
    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    /// Command not yet implemented.
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Configuration error.
    #[error("Config error: {message}")]
    Config {
        /// Error message.
        message: String,
    },
}

/// Result type alias using `NjallaError`.
pub type Result<T> = std::result::Result<T, NjallaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_missing_token() {
        let err = NjallaError::MissingToken;
        assert_eq!(
            err.to_string(),
            "No API token found. Set NJALLA_API_TOKEN or add api_token to ./config.toml"
        );
    }

    #[test]
    fn error_display_api_error() {
        let err = NjallaError::Api {
            message: "Invalid token".to_string(),
        };
        assert_eq!(err.to_string(), "API error: Invalid token");
    }

    #[test]
    fn error_display_domain_not_available() {
        let err = NjallaError::DomainNotAvailable("example.com".to_string());
        assert_eq!(err.to_string(), "Domain not available: example.com");
    }

    #[test]
    fn error_display_timeout() {
        let err = NjallaError::RegistrationTimeout {
            domain: "example.com".to_string(),
            timeout_secs: 300,
        };
        assert_eq!(
            err.to_string(),
            "Registration timeout for example.com after 300s"
        );
    }
}
