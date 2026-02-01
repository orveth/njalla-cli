//! Configuration management for njalla-cli.
//!
//! Configuration is loaded from (in order of precedence):
//! 1. Environment variable `NJALLA_API_TOKEN`
//! 2. Config file at `~/.config/njalla/config.toml`
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

/// Application name for config directory.
const APP_NAME: &str = "njalla";

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
    /// 2. Config file `~/.config/njalla/config.toml`
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
        let path = Self::config_path()?;

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
    pub fn api_token(&self) -> Result<&str> {
        self.api_token
            .as_deref()
            .ok_or(NjallaError::MissingToken)
    }

    /// Get the config file path.
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| NjallaError::Config {
            message: "Could not determine config directory".to_string(),
        })?;

        Ok(config_dir.join(APP_NAME).join(CONFIG_FILE))
    }

    /// Get the config directory path.
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| NjallaError::Config {
            message: "Could not determine config directory".to_string(),
        })?;

        Ok(config_dir.join(APP_NAME))
    }

    /// Initialize config file with placeholder if it doesn't exist.
    pub fn init() -> Result<PathBuf> {
        let dir = Self::config_dir()?;
        let path = Self::config_path()?;

        if path.exists() {
            return Ok(path);
        }

        // Create directory if needed
        fs::create_dir_all(&dir).map_err(|e| NjallaError::Config {
            message: format!("Failed to create config directory: {e}"),
        })?;

        // Write template
        let template = r#"# Njalla CLI Configuration
# Get your API token from: https://njal.la → Settings → API

api_token = ""
"#;

        fs::write(&path, template).map_err(|e| NjallaError::Config {
            message: format!("Failed to write config file: {e}"),
        })?;

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_path_exists() {
        // Should not panic
        let result = Config::config_path();
        assert!(result.is_ok());
    }

    #[test]
    fn default_config_has_no_token() {
        let config = Config::default();
        assert!(config.api_token.is_none());
    }

    #[test]
    fn env_var_overrides_file() {
        std::env::set_var("NJALLA_API_TOKEN", "test-from-env");
        let config = Config::load().unwrap();
        assert_eq!(config.api_token().unwrap(), "test-from-env");
        std::env::remove_var("NJALLA_API_TOKEN");
    }
}
