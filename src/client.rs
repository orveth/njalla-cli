//! Njalla API client.
//!
//! Handles all communication with the Njalla API.

use crate::config::Config;
use crate::error::{NjallaError, Result};
use crate::types::{
    ApiRequest, ApiResponse, Domain, MarketDomain, Payment, PaymentMethod, Record, TaskStatus,
    Transaction, TransactionsResult, WalletBalance,
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

    /// Debug mode - print raw responses.
    debug: bool,
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
    pub fn new(debug: bool) -> Result<Self> {
        let config = Config::load()?;
        let token = config.api_token()?.to_string();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        Ok(Self {
            client,
            token,
            base_url: API_ENDPOINT.to_string(),
            debug,
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
            debug: false,
        })
    }

    /// Make an API request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the API returns an error.
    async fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let request = ApiRequest {
            method: method.to_string(),
            params: params.clone(),
        };

        if self.debug {
            eprintln!("[DEBUG] Request: {method} {params:?}");
        }

        let response = self
            .client
            .post(&self.base_url)
            .header("Authorization", format!("Njalla {}", self.token))
            .json(&request)
            .send()
            .await?;

        let response_text = response.text().await?;

        if self.debug {
            eprintln!("[DEBUG] Response: {response_text}");
        }

        let api_response: ApiResponse<T> = serde_json::from_str(&response_text)?;

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

    // ========================================================================
    // Wallet Methods
    // ========================================================================

    /// Get wallet balance.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get_balance(&self) -> Result<WalletBalance> {
        self.request("get-balance", serde_json::json!({})).await
    }

    /// Add payment to refill wallet.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount in EUR (5 or multiple of 15, max 300)
    /// * `via` - Payment method (bitcoin)
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or parameters are invalid.
    pub async fn add_payment(&self, amount: i32, via: PaymentMethod) -> Result<Payment> {
        self.request(
            "add-payment",
            serde_json::json!({
                "amount": amount,
                "via": via.to_string()
            }),
        )
        .await
    }

    /// Get details about a payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the payment is not found.
    pub async fn get_payment(&self, id: &str) -> Result<Payment> {
        self.request("get-payment", serde_json::json!({ "id": id }))
            .await
    }

    /// List transactions from the last 90 days.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let result: TransactionsResult = self
            .request("list-transactions", serde_json::json!({}))
            .await?;
        Ok(result.transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PaymentMethod;
    use wiremock::matchers::{body_json_string, header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

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

    #[tokio::test]
    async fn get_balance_returns_balance() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(body_json_string(r#"{"method":"get-balance","params":{}}"#))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": { "balance": 42 }
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let balance = client.get_balance().await.unwrap();

        assert_eq!(balance.balance, 42);
    }

    #[tokio::test]
    async fn add_payment_sends_correct_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(body_json_string(
                r#"{"method":"add-payment","params":{"amount":15,"via":"bitcoin"}}"#,
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {
                    "id": "pay123",
                    "amount": 15,
                    "address": "bc1qtest..."
                }
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let payment = client
            .add_payment(15, PaymentMethod::Bitcoin)
            .await
            .unwrap();

        assert_eq!(payment.amount, 15);
        assert_eq!(payment.id, Some("pay123".to_string()));
        assert_eq!(payment.address, Some("bc1qtest...".to_string()));
    }

    #[tokio::test]
    async fn get_payment_sends_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(body_json_string(
                r#"{"method":"get-payment","params":{"id":"pay456"}}"#,
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {
                    "id": "pay456",
                    "amount": 30,
                    "status": "completed"
                }
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let payment = client.get_payment("pay456").await.unwrap();

        assert_eq!(payment.id, Some("pay456".to_string()));
        assert_eq!(payment.status, Some("completed".to_string()));
    }

    #[tokio::test]
    async fn list_transactions_unwraps_result() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(body_json_string(
                r#"{"method":"list-transactions","params":{}}"#,
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "result": {
                    "transactions": [
                        {
                            "id": "tx1",
                            "amount": 50,
                            "status": "Added 50 € via Bitcoin",
                            "completed": "2026-01-15",
                            "pdf": "https://njal.la/invoice/tx1/"
                        },
                        {
                            "id": "tx2",
                            "amount": 15,
                            "status": "Waiting for transaction",
                            "uri": "bitcoin:bc1qtest",
                            "address": "bc1qtest",
                            "currency": "EUR",
                            "amount_btc": "0.0002"
                        }
                    ]
                }
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let transactions = client.list_transactions().await.unwrap();

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].id, "tx1");
        assert_eq!(transactions[0].amount, 50);
        assert_eq!(transactions[0].completed, Some("2026-01-15".to_string()));
        assert_eq!(transactions[1].status, "Waiting for transaction");
        assert!(transactions[1].completed.is_none());
    }
}
