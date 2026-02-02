//! Error types for njalla-cli.

use std::fmt;

/// All errors that can occur in njalla-cli.
#[derive(Debug)]
pub enum NjallaError {
    /// API token not found.
    MissingToken,

    /// HTTP request failed.
    Request(bitreq::Error),

    /// API returned an error response.
    Api {
        /// Error message from the API.
        message: String,
    },

    /// Domain is not available for registration.
    DomainNotAvailable(String),

    /// Registration timed out waiting for completion.
    RegistrationTimeout {
        /// Domain being registered.
        domain: String,
        /// Timeout in seconds.
        timeout_secs: u64,
    },

    /// JSON parsing failed.
    Parse(serde_json::Error),

    /// Configuration error.
    Config {
        /// Error message.
        message: String,
    },
}

impl fmt::Display for NjallaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingToken => write!(
                f,
                "No API token found. Set NJALLA_API_TOKEN or add api_token to ./config.toml"
            ),
            Self::Request(e) => write!(f, "Request failed: {e}"),
            Self::Api { message } => write!(f, "API error: {message}"),
            Self::DomainNotAvailable(s) => write!(f, "Domain not available: {s}"),
            Self::RegistrationTimeout {
                domain,
                timeout_secs,
            } => write!(f, "Registration timeout for {domain} after {timeout_secs}s"),
            Self::Parse(e) => write!(f, "Failed to parse response: {e}"),
            Self::Config { message } => write!(f, "Config error: {message}"),
        }
    }
}

impl std::error::Error for NjallaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Request(e) => Some(e),
            Self::Parse(e) => Some(e),
            _ => None,
        }
    }
}

impl From<bitreq::Error> for NjallaError {
    fn from(err: bitreq::Error) -> Self {
        Self::Request(err)
    }
}

impl From<serde_json::Error> for NjallaError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
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
