//! Njalla API client.
//!
//! Handles all communication with the Njalla API.

use crate::config::Config;
use crate::error::{NjallaError, Result};
use crate::types::{
    ApiRequest, ApiResponse, Domain, MarketDomain, Record, TaskStatus,
};

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

    /// Base URL for API requests.
    base_url: String,
}

impl NjallaClient {
    /// Create a new client from configuration.
    ///
    /// Loads token from (in order of precedence):
    /// 1. `NJALLA_API_TOKEN` environment variable
    /// 2. Config file at `./config.toml`
    ///
    /// # Errors
    ///
    /// Returns `NjallaError::MissingToken` if no token is configured.
    /// Returns `NjallaError::Request` if the HTTP client fails to build.
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let token = config.api_token()?.to_string();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        Ok(Self {
            client,
            token,
            base_url: API_ENDPOINT.to_string(),
        })
    }

    /// Create a new client with a custom base URL (for testing).
    #[cfg(test)]
    pub fn with_base_url(token: &str, base_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        Ok(Self {
            client,
            token: token.to_string(),
            base_url: base_url.to_string(),
        })
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
            .post(&self.base_url)
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
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[allow(dead_code, clippy::unused_async)]
    pub async fn list_domains(&self) -> Result<Vec<Domain>> {
        // TODO: Implement in Phase 2
        Err(NjallaError::NotImplemented("list_domains".to_string()))
    }

    /// Get detailed info for a specific domain.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the domain is not found.
    #[allow(dead_code, clippy::unused_async)]
    pub async fn get_domain(&self, _domain: &str) -> Result<Domain> {
        // TODO: Implement in Phase 2
        Err(NjallaError::NotImplemented("get_domain".to_string()))
    }

    /// Search for available domains.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[allow(dead_code, clippy::unused_async)]
    pub async fn find_domains(&self, _query: &str) -> Result<Vec<MarketDomain>> {
        // TODO: Implement in Phase 2
        Err(NjallaError::NotImplemented("find_domains".to_string()))
    }

    // ========================================================================
    // Registration Methods (Phase 4)
    // ========================================================================

    /// Register a domain (returns task ID).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the domain is unavailable.
    #[allow(dead_code, clippy::unused_async)]
    pub async fn register_domain(&self, _domain: &str, _years: i32) -> Result<String> {
        // TODO: Implement in Phase 4
        Err(NjallaError::NotImplemented("register_domain".to_string()))
    }

    /// Check task status.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the task is not found.
    #[allow(dead_code, clippy::unused_async)]
    pub async fn check_task(&self, _task_id: &str) -> Result<TaskStatus> {
        // TODO: Implement in Phase 4
        Err(NjallaError::NotImplemented("check_task".to_string()))
    }

    // ========================================================================
    // DNS Methods (Phase 5)
    // ========================================================================

    /// List DNS records for a domain.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[allow(dead_code, clippy::unused_async)]
    pub async fn list_records(&self, _domain: &str) -> Result<Vec<Record>> {
        // TODO: Implement in Phase 5
        Err(NjallaError::NotImplemented("list_records".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json_string, header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn new_client_requires_token() {
        // Save original value if present
        let original = std::env::var("NJALLA_API_TOKEN").ok();

        // Test: without token should fail
        std::env::remove_var("NJALLA_API_TOKEN");
        let result = NjallaClient::new();
        assert!(
            matches!(result, Err(NjallaError::MissingToken)),
            "expected MissingToken error without token"
        );

        // Test: with token should succeed
        std::env::set_var("NJALLA_API_TOKEN", "test-token");
        let result = NjallaClient::new();
        assert!(result.is_ok(), "expected Ok with token set");

        // Restore original value
        match original {
            Some(val) => std::env::set_var("NJALLA_API_TOKEN", val),
            None => std::env::remove_var("NJALLA_API_TOKEN"),
        }
    }

    #[tokio::test]
    async fn request_sends_correct_headers() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(header("Authorization", "Njalla test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": { "domains": [] }
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = NjallaClient::with_base_url("test-token", &mock_server.uri()).unwrap();

        let result: std::result::Result<serde_json::Value, _> =
            client.request("list-domains", serde_json::json!({})).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn request_handles_api_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "error": { "message": "Invalid token" }
            })))
            .mount(&mock_server)
            .await;

        let client = NjallaClient::with_base_url("bad-token", &mock_server.uri()).unwrap();

        let result: std::result::Result<serde_json::Value, _> =
            client.request("list-domains", serde_json::json!({})).await;

        assert!(matches!(result, Err(NjallaError::Api { message }) if message == "Invalid token"));
    }

    #[tokio::test]
    async fn request_sends_correct_body() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(body_json_string(
                r#"{"method":"test-method","params":{"key":"value"}}"#,
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {}
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();

        let result: std::result::Result<serde_json::Value, _> = client
            .request("test-method", serde_json::json!({"key": "value"}))
            .await;

        assert!(result.is_ok());
    }
}
