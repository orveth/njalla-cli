//! Njalla API client.
//!
//! Handles all communication with the Njalla API.

use crate::error::{NjallaError, Result};
use crate::types::*;

/// Njalla API endpoint.
pub const API_ENDPOINT: &str = "https://njal.la/api/1/";

/// Default request timeout in seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Njalla API client.
pub struct NjallaClient {
    /// HTTP client.
    client: reqwest::Client,

    /// API token.
    token: String,
}

impl NjallaClient {
    /// Create a new client from the `NJALLA_API_TOKEN` environment variable.
    ///
    /// # Errors
    ///
    /// Returns `NjallaError::MissingToken` if the environment variable is not set.
    pub fn new() -> Result<Self> {
        let token =
            std::env::var("NJALLA_API_TOKEN").map_err(|_| NjallaError::MissingToken)?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        Ok(Self { client, token })
    }

    /// Make an API request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    #[allow(dead_code)]
    async fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let request = ApiRequest {
            method: method.to_string(),
            params,
        };

        let response = self
            .client
            .post(API_ENDPOINT)
            .header("Authorization", format!("Njalla {}", self.token))
            .json(&request)
            .send()
            .await?;

        let api_response: ApiResponse<T> = response.json().await?;

        if let Some(error) = api_response.error {
            return Err(NjallaError::Api {
                message: error.message,
            });
        }

        api_response.result.ok_or_else(|| NjallaError::Api {
            message: "Missing result in response".to_string(),
        })
    }

    // ========================================================================
    // Domain Methods (Phase 2-3)
    // ========================================================================

    /// List all domains in the account.
    #[allow(dead_code)]
    pub async fn list_domains(&self) -> Result<Vec<Domain>> {
        // TODO: Implement in Phase 2
        Err(NjallaError::NotImplemented("list_domains".to_string()))
    }

    /// Get detailed info for a specific domain.
    #[allow(dead_code)]
    pub async fn get_domain(&self, _domain: &str) -> Result<Domain> {
        // TODO: Implement in Phase 2
        Err(NjallaError::NotImplemented("get_domain".to_string()))
    }

    /// Search for available domains.
    #[allow(dead_code)]
    pub async fn find_domains(&self, _query: &str) -> Result<Vec<MarketDomain>> {
        // TODO: Implement in Phase 2
        Err(NjallaError::NotImplemented("find_domains".to_string()))
    }

    // ========================================================================
    // Registration Methods (Phase 4)
    // ========================================================================

    /// Register a domain (returns task ID).
    #[allow(dead_code)]
    pub async fn register_domain(&self, _domain: &str, _years: i32) -> Result<String> {
        // TODO: Implement in Phase 4
        Err(NjallaError::NotImplemented("register_domain".to_string()))
    }

    /// Check task status.
    #[allow(dead_code)]
    pub async fn check_task(&self, _task_id: &str) -> Result<TaskStatus> {
        // TODO: Implement in Phase 4
        Err(NjallaError::NotImplemented("check_task".to_string()))
    }

    // ========================================================================
    // DNS Methods (Phase 5)
    // ========================================================================

    /// List DNS records for a domain.
    #[allow(dead_code)]
    pub async fn list_records(&self, _domain: &str) -> Result<Vec<Record>> {
        // TODO: Implement in Phase 5
        Err(NjallaError::NotImplemented("list_records".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client_missing_token() {
        // Ensure token is not set
        std::env::remove_var("NJALLA_API_TOKEN");

        let result = NjallaClient::new();
        assert!(matches!(result, Err(NjallaError::MissingToken)));
    }

    #[test]
    fn new_client_with_token() {
        std::env::set_var("NJALLA_API_TOKEN", "test-token");

        let result = NjallaClient::new();
        assert!(result.is_ok());

        // Clean up
        std::env::remove_var("NJALLA_API_TOKEN");
    }
}
