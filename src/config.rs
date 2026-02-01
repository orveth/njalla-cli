//! Configuration management for njalla-cli.
//!
//! Configuration is loaded from (in order of precedence):
//! 1. Environment variable `NJALLA_API_TOKEN`
//! 2. Config file at `./config.toml` (project directory)
//!
//! # Config File Format
//!
//! ```toml
//! api_token = "your-api-token-here"
//! ```

use crate::error::{NjallaError, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Configuration file name.
const CONFIG_FILE: &str = "config.toml";

/// Configuration structure.
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// Njalla API token.
    pub api_token: Option<String>,
}

impl Config {
    /// Load configuration from file and environment.
    ///
    /// Priority:
    /// 1. `NJALLA_API_TOKEN` environment variable (highest)
    /// 2. Config file `./config.toml` (project directory)
    ///
    /// # Errors
    ///
    /// Returns an error if the config file exists but cannot be read or parsed.
    pub fn load() -> Result<Self> {
        let path = PathBuf::from(CONFIG_FILE);

        // Start with config file (if exists)
        let mut config = if path.exists() {
            let contents = fs::read_to_string(&path).map_err(|e| NjallaError::Config {
                message: format!("Failed to read config file: {e}"),
            })?;
            toml::from_str(&contents).map_err(|e| NjallaError::Config {
                message: format!("Failed to parse config file: {e}"),
            })?
        } else {
            Self::default()
        };

        // Override with environment variable
        if let Ok(token) = std::env::var("NJALLA_API_TOKEN") {
            if !token.is_empty() {
                config.api_token = Some(token);
            }
        }

        Ok(config)
    }

    /// Get the API token, returning an error if not configured.
    ///
    /// # Errors
    ///
    /// Returns `NjallaError::MissingToken` if no API token is configured.
    pub fn api_token(&self) -> Result<&str> {
        self.api_token
            .as_deref()
            .ok_or(NjallaError::MissingToken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_no_token() {
        let config = Config::default();
        assert!(config.api_token.is_none());
    }

    #[test]
    fn api_token_returns_error_when_missing() {
        let config = Config::default();
        assert!(matches!(config.api_token(), Err(NjallaError::MissingToken)));
    }

    #[test]
    fn api_token_returns_token_when_present() {
        let config = Config {
            api_token: Some("test-token".to_string()),
        };
        assert_eq!(config.api_token().unwrap(), "test-token");
    }
}
