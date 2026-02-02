//! Njalla API client.
//!
//! Handles all communication with the Njalla API.

use crate::config::Config;
use crate::error::{NjallaError, Result};
use crate::types::{
    ApiRequest, ApiResponse, Domain, DomainsResult, MarketDomain, MarketDomainsResult, Payment,
    PaymentMethod, Record, RecordsResult, RegisterResult, TaskStatus, Transaction,
    TransactionsResult, WalletBalance,
};

/// Njalla API endpoint.
pub const API_ENDPOINT: &str = "https://njal.la/api/1/";

/// Default request timeout in seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Njalla API client.
pub struct NjallaClient {
    /// HTTP client.
    client: reqwest::blocking::Client,

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

        let client = reqwest::blocking::Client::builder()
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
        let client = reqwest::blocking::Client::builder()
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
    fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        if self.debug {
            eprintln!("[DEBUG] Request: {method} {params:?}");
        }

        let request = ApiRequest {
            method: method.to_string(),
            params,
        };

        let response = self
            .client
            .post(&self.base_url)
            .header("Authorization", format!("Njalla {}", self.token))
            .json(&request)
            .send()?;

        let response_text = response.text()?;

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
    pub fn list_domains(&self) -> Result<Vec<Domain>> {
        let result: DomainsResult = self.request("list-domains", serde_json::json!({}))?;
        Ok(result.domains)
    }

    /// Get detailed info for a specific domain.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the domain is not found.
    pub fn get_domain(&self, domain: &str) -> Result<Domain> {
        self.request("get-domain", serde_json::json!({ "domain": domain }))
    }

    /// Search for available domains.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub fn find_domains(&self, query: &str) -> Result<Vec<MarketDomain>> {
        let result: MarketDomainsResult =
            self.request("find-domains", serde_json::json!({ "query": query }))?;
        Ok(result.domains)
    }

    // ========================================================================
    // Registration Methods (Phase 4)
    // ========================================================================

    /// Register a domain (returns task ID).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the domain is unavailable.
    pub fn register_domain(&self, domain: &str, years: i32) -> Result<String> {
        let result: RegisterResult = self.request(
            "register-domain",
            serde_json::json!({ "domain": domain, "years": years }),
        )?;
        Ok(result.task)
    }

    /// Check task status.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the task is not found.
    pub fn check_task(&self, task_id: &str) -> Result<TaskStatus> {
        self.request("check-task", serde_json::json!({ "id": task_id }))
    }

    // ========================================================================
    // DNS Methods (Phase 5)
    // ========================================================================

    /// List DNS records for a domain.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub fn list_records(&self, domain: &str) -> Result<Vec<Record>> {
        let result: RecordsResult =
            self.request("list-records", serde_json::json!({ "domain": domain }))?;
        Ok(result.records)
    }

    // ========================================================================
    // Wallet Methods
    // ========================================================================

    /// Get wallet balance.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub fn get_balance(&self) -> Result<WalletBalance> {
        self.request("get-balance", serde_json::json!({}))
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
    pub fn add_payment(&self, amount: i32, via: PaymentMethod) -> Result<Payment> {
        self.request(
            "add-payment",
            serde_json::json!({
                "amount": amount,
                "via": via.to_string()
            }),
        )
    }

    /// Get details about a payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or the payment is not found.
    pub fn get_payment(&self, id: &str) -> Result<Payment> {
        self.request("get-payment", serde_json::json!({ "id": id }))
    }

    /// List transactions from the last 90 days.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let result: TransactionsResult =
            self.request("list-transactions", serde_json::json!({}))?;
        Ok(result.transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PaymentMethod;
    use std::sync::LazyLock;
    use wiremock::matchers::{body_json_string, header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // wiremock requires tokio runtime for MockServer
    static RT: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    });

    fn mock_server() -> MockServer {
        RT.block_on(MockServer::start())
    }

    fn mount(server: &MockServer, mock: Mock) {
        RT.block_on(mock.mount(server));
    }

    #[test]
    fn request_sends_correct_headers() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(header("Authorization", "Njalla test-token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": { "domains": [] }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("test-token", &mock_server.uri()).unwrap();

        let result: std::result::Result<serde_json::Value, _> =
            client.request("list-domains", serde_json::json!({}));

        assert!(result.is_ok());
    }

    #[test]
    fn request_handles_api_error() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST")).respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "error": { "message": "Invalid token" }
                })),
            ),
        );

        let client = NjallaClient::with_base_url("bad-token", &mock_server.uri()).unwrap();

        let result: std::result::Result<serde_json::Value, _> =
            client.request("list-domains", serde_json::json!({}));

        assert!(matches!(result, Err(NjallaError::Api { message }) if message == "Invalid token"));
    }

    #[test]
    fn request_sends_correct_body() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"test-method","params":{"key":"value"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {}
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();

        let result: std::result::Result<serde_json::Value, _> =
            client.request("test-method", serde_json::json!({"key": "value"}));

        assert!(result.is_ok());
    }

    #[test]
    fn get_balance_returns_balance() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(r#"{"method":"get-balance","params":{}}"#))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": { "balance": 42 }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let balance = client.get_balance().unwrap();

        assert_eq!(balance.balance, 42);
    }

    #[test]
    fn add_payment_sends_correct_params() {
        let mock_server = mock_server();

        mount(
            &mock_server,
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
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let payment = client.add_payment(15, PaymentMethod::Bitcoin).unwrap();

        assert_eq!(payment.amount, 15);
        assert_eq!(payment.id, Some("pay123".to_string()));
        assert_eq!(payment.address, Some("bc1qtest...".to_string()));
    }

    #[test]
    fn get_payment_sends_id() {
        let mock_server = mock_server();

        mount(
            &mock_server,
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
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let payment = client.get_payment("pay456").unwrap();

        assert_eq!(payment.id, Some("pay456".to_string()));
        assert_eq!(payment.status, Some("completed".to_string()));
    }

    #[test]
    fn list_transactions_unwraps_result() {
        let mock_server = mock_server();

        mount(
            &mock_server,
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
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let transactions = client.list_transactions().unwrap();

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].id, "tx1");
        assert_eq!(transactions[0].amount, 50);
        assert_eq!(transactions[0].completed, Some("2026-01-15".to_string()));
        assert_eq!(transactions[1].status, "Waiting for transaction");
        assert!(transactions[1].completed.is_none());
    }

    // ========================================================================
    // Domain Methods Tests
    // ========================================================================

    #[test]
    fn list_domains_returns_domains() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(r#"{"method":"list-domains","params":{}}"#))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "domains": [
                            {
                                "name": "example.com",
                                "status": "active",
                                "expiry": "2027-01-15T00:00:00Z",
                                "locked": false
                            },
                            {
                                "name": "test.org",
                                "status": "pending"
                            }
                        ]
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let domains = client.list_domains().unwrap();

        assert_eq!(domains.len(), 2);
        assert_eq!(domains[0].name, "example.com");
        assert_eq!(domains[0].status, "active");
        assert_eq!(domains[1].name, "test.org");
        assert_eq!(domains[1].status, "pending");
    }

    #[test]
    fn list_domains_empty() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(r#"{"method":"list-domains","params":{}}"#))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": { "domains": [] }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let domains = client.list_domains().unwrap();

        assert!(domains.is_empty());
    }

    #[test]
    fn get_domain_returns_details() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"get-domain","params":{"domain":"example.com"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "name": "example.com",
                        "status": "active",
                        "expiry": "2027-01-15T00:00:00Z",
                        "locked": true,
                        "mailforwarding": false,
                        "max_nameservers": 4
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let domain = client.get_domain("example.com").unwrap();

        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.status, "active");
        assert_eq!(domain.locked, Some(true));
        assert_eq!(domain.mailforwarding, Some(false));
        assert_eq!(domain.max_nameservers, Some(4));
    }

    #[test]
    fn get_domain_not_found() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"get-domain","params":{"domain":"notfound.com"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "error": { "message": "Domain not found" }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let result = client.get_domain("notfound.com");

        assert!(matches!(result, Err(NjallaError::Api { message }) if message == "Domain not found"));
    }

    #[test]
    fn find_domains_returns_search_results() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"find-domains","params":{"query":"example"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "domains": [
                            { "name": "example.com", "status": "available", "price": 15 },
                            { "name": "example.net", "status": "taken", "price": 15 },
                            { "name": "example.org", "status": "available", "price": 18 }
                        ]
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let results = client.find_domains("example").unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].name, "example.com");
        assert_eq!(results[0].status, "available");
        assert_eq!(results[0].price, 15);
        assert_eq!(results[1].status, "taken");
        assert_eq!(results[2].price, 18);
    }

    #[test]
    fn list_records_returns_dns_records() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"list-records","params":{"domain":"example.com"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "records": [
                            {
                                "id": "rec1",
                                "name": "@",
                                "type": "A",
                                "content": "192.0.2.1",
                                "ttl": 10800,
                                "prio": null
                            },
                            {
                                "id": "rec2",
                                "name": "www",
                                "type": "CNAME",
                                "content": "example.com",
                                "ttl": 3600,
                                "prio": null
                            },
                            {
                                "id": "rec3",
                                "name": "@",
                                "type": "MX",
                                "content": "mail.example.com",
                                "ttl": 10800,
                                "prio": 10
                            }
                        ]
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let records = client.list_records("example.com").unwrap();

        assert_eq!(records.len(), 3);
        assert_eq!(records[0].id, "rec1");
        assert_eq!(records[0].name, "@");
        assert_eq!(records[0].record_type, "A");
        assert_eq!(records[0].content, "192.0.2.1");
        assert_eq!(records[0].ttl, 10800);
        assert!(records[0].priority.is_none());
        assert_eq!(records[2].priority, Some(10));
    }

    // ========================================================================
    // Registration Methods Tests
    // ========================================================================

    #[test]
    fn register_domain_returns_task_id() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"register-domain","params":{"domain":"newdomain.com","years":1}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": { "task": "task-abc123" }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let task_id = client.register_domain("newdomain.com", 1).unwrap();

        assert_eq!(task_id, "task-abc123");
    }

    #[test]
    fn register_domain_insufficient_funds() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"register-domain","params":{"domain":"expensive.com","years":2}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "error": { "message": "Insufficient funds" }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let result = client.register_domain("expensive.com", 2);

        assert!(matches!(result, Err(NjallaError::Api { message }) if message == "Insufficient funds"));
    }

    #[test]
    fn check_task_returns_completed_status() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"check-task","params":{"id":"task-abc123"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "task-abc123",
                        "status": "completed"
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let status = client.check_task("task-abc123").unwrap();

        assert_eq!(status.id, "task-abc123");
        assert_eq!(status.status, "completed");
    }

    #[test]
    fn check_task_returns_pending_status() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"check-task","params":{"id":"task-xyz789"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "task-xyz789",
                        "status": "pending"
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let status = client.check_task("task-xyz789").unwrap();

        assert_eq!(status.id, "task-xyz789");
        assert_eq!(status.status, "pending");
    }

    #[test]
    fn check_task_returns_failed_status() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"check-task","params":{"id":"task-fail"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "task-fail",
                        "status": "failed"
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri()).unwrap();
        let status = client.check_task("task-fail").unwrap();

        assert_eq!(status.id, "task-fail");
        assert_eq!(status.status, "failed");
    }
}
