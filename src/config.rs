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
    #[allow(clippy::unnecessary_wraps)]
    pub fn load() -> Result<Self> {
        // Start with config file (if exists)
        let mut config = Self::load_from_file().unwrap_or_default();

        // Override with environment variable
        if let Ok(token) = std::env::var("NJALLA_API_TOKEN") {
            if !token.is_empty() {
                config.api_token = Some(token);
            }
        }

        Ok(config)
    }

    /// Load configuration from file only.
    fn load_from_file() -> Result<Self> {
        let path = PathBuf::from(CONFIG_FILE);

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path).map_err(|e| NjallaError::Config {
            message: format!("Failed to read config file: {e}"),
        })?;

        toml::from_str(&contents).map_err(|e| NjallaError::Config {
            message: format!("Failed to parse config file: {e}"),
        })
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
    fn config_loading_and_env_var() {
        // Save original value
        let original = std::env::var("NJALLA_API_TOKEN").ok();

        // Test: without env var, load returns default (no token)
        std::env::remove_var("NJALLA_API_TOKEN");
        let config = Config::load().unwrap();
        assert!(config.api_token.is_none());

        // Test: env var overrides file
        std::env::set_var("NJALLA_API_TOKEN", "test-from-env");
        let config = Config::load().unwrap();
        assert_eq!(config.api_token().unwrap(), "test-from-env");

        // Restore original value
        match original {
            Some(val) => std::env::set_var("NJALLA_API_TOKEN", val),
            None => std::env::remove_var("NJALLA_API_TOKEN"),
        }
    }
}
