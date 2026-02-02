//! Njalla API client.
//!
//! Handles all communication with the Njalla API.

use crate::config::Config;
use crate::error::{NjallaError, Result};
use crate::types::{
    AddRecordParams, ApiRequest, ApiResponse, Domain, DomainsResult, EditRecordParams,
    MarketDomain, MarketDomainsResult, Payment, PaymentMethod, Record, RecordsResult,
    RegisterResult, TaskStatus, Transaction, TransactionsResult, WalletBalance,
};

/// Njalla API endpoint.
pub const API_ENDPOINT: &str = "https://njal.la/api/1/";

/// Default request timeout in seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Njalla API client.
pub struct NjallaClient {
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
    pub fn new(debug: bool) -> Result<Self> {
        let config = Config::load()?;
        let token = config.api_token()?.to_string();

        Ok(Self {
            token,
            base_url: API_ENDPOINT.to_string(),
            debug,
        })
    }

    /// Create a new client with a custom base URL (for testing).
    #[cfg(test)]
    pub fn with_base_url(token: &str, base_url: &str) -> Self {
        Self {
            token: token.to_string(),
            base_url: base_url.to_string(),
            debug: false,
        }
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
        let request_body = ApiRequest {
            method: method.to_string(),
            params,
        };

        let body = serde_json::to_string(&request_body)?;

        if self.debug {
            eprintln!("[DEBUG] Request: {method} {body}");
        }

        let response = bitreq::post(&self.base_url)
            .with_header("Authorization", format!("Njalla {}", self.token))
            .with_header("Content-Type", "application/json")
            .with_body(body.into_bytes())
            .with_timeout(DEFAULT_TIMEOUT_SECS)
            .send()?;

        let response_text = response.as_str()?;

        if self.debug {
            eprintln!("[DEBUG] Response: {response_text}");
        }

        let api_response: ApiResponse<T> = serde_json::from_str(response_text)?;

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

    /// Add a DNS record to a domain.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[allow(clippy::missing_panics_doc)]
    pub fn add_record(&self, params: &AddRecordParams) -> Result<Record> {
        let mut json_params = serde_json::json!({
            "domain": params.domain,
            "type": params.record_type,
            "name": params.name,
        });

        // Safe: json! macro always creates an object when given object syntax
        let obj = json_params.as_object_mut().expect("json object");

        if let Some(content) = &params.content {
            obj.insert("content".to_string(), serde_json::json!(content));
        }
        if let Some(ttl) = params.ttl {
            obj.insert("ttl".to_string(), serde_json::json!(ttl));
        }
        if let Some(prio) = params.priority {
            obj.insert("prio".to_string(), serde_json::json!(prio));
        }
        if let Some(weight) = params.weight {
            obj.insert("weight".to_string(), serde_json::json!(weight));
        }
        if let Some(port) = params.port {
            obj.insert("port".to_string(), serde_json::json!(port));
        }
        if let Some(target) = &params.target {
            obj.insert("target".to_string(), serde_json::json!(target));
        }
        if let Some(value) = &params.value {
            obj.insert("value".to_string(), serde_json::json!(value));
        }
        if let Some(ssh_algorithm) = params.ssh_algorithm {
            obj.insert("ssh_algorithm".to_string(), serde_json::json!(ssh_algorithm));
        }
        if let Some(ssh_type) = params.ssh_type {
            obj.insert("ssh_type".to_string(), serde_json::json!(ssh_type));
        }

        self.request("add-record", json_params)
    }

    /// Edit an existing DNS record.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[allow(clippy::missing_panics_doc)]
    pub fn edit_record(&self, params: &EditRecordParams) -> Result<Record> {
        let mut json_params = serde_json::json!({
            "domain": params.domain,
            "id": params.id,
        });

        // Safe: json! macro always creates an object when given object syntax
        let obj = json_params.as_object_mut().expect("json object");

        if let Some(name) = &params.name {
            obj.insert("name".to_string(), serde_json::json!(name));
        }
        if let Some(content) = &params.content {
            obj.insert("content".to_string(), serde_json::json!(content));
        }
        if let Some(ttl) = params.ttl {
            obj.insert("ttl".to_string(), serde_json::json!(ttl));
        }
        if let Some(prio) = params.priority {
            obj.insert("prio".to_string(), serde_json::json!(prio));
        }
        if let Some(weight) = params.weight {
            obj.insert("weight".to_string(), serde_json::json!(weight));
        }
        if let Some(port) = params.port {
            obj.insert("port".to_string(), serde_json::json!(port));
        }
        if let Some(target) = &params.target {
            obj.insert("target".to_string(), serde_json::json!(target));
        }
        if let Some(value) = &params.value {
            obj.insert("value".to_string(), serde_json::json!(value));
        }
        if let Some(ssh_algorithm) = params.ssh_algorithm {
            obj.insert("ssh_algorithm".to_string(), serde_json::json!(ssh_algorithm));
        }
        if let Some(ssh_type) = params.ssh_type {
            obj.insert("ssh_type".to_string(), serde_json::json!(ssh_type));
        }

        self.request("edit-record", json_params)
    }

    /// Remove a DNS record from a domain.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub fn remove_record(&self, domain: &str, id: &str) -> Result<()> {
        let _: serde_json::Value = self.request(
            "remove-record",
            serde_json::json!({ "domain": domain, "id": id }),
        )?;
        Ok(())
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

        let client = NjallaClient::with_base_url("test-token", &mock_server.uri());

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

        let client = NjallaClient::with_base_url("bad-token", &mock_server.uri());

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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());

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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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
        use crate::types::RecordType;

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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let records = client.list_records("example.com").unwrap();

        assert_eq!(records.len(), 3);
        assert_eq!(records[0].id, "rec1");
        assert_eq!(records[0].name, "@");
        assert_eq!(records[0].record_type, RecordType::A);
        assert_eq!(records[0].content, Some("192.0.2.1".to_string()));
        assert_eq!(records[0].ttl, Some(10800));
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
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

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let status = client.check_task("task-fail").unwrap();

        assert_eq!(status.id, "task-fail");
        assert_eq!(status.status, "failed");
    }

    // ========================================================================
    // DNS CRUD Methods Tests
    // ========================================================================

    #[test]
    fn add_record_a_type() {
        use crate::types::RecordType;

        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"add-record","params":{"domain":"example.com","type":"A","name":"@","content":"1.2.3.4","ttl":3600}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "rec123",
                        "name": "@",
                        "type": "A",
                        "content": "1.2.3.4",
                        "ttl": 3600
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let params = AddRecordParams {
            domain: "example.com".to_string(),
            record_type: RecordType::A,
            name: "@".to_string(),
            content: Some("1.2.3.4".to_string()),
            ttl: Some(3600),
            priority: None,
            weight: None,
            port: None,
            target: None,
            value: None,
            ssh_algorithm: None,
            ssh_type: None,
        };
        let record = client.add_record(&params).unwrap();

        assert_eq!(record.id, "rec123");
        assert_eq!(record.name, "@");
        assert_eq!(record.record_type, RecordType::A);
        assert_eq!(record.content, Some("1.2.3.4".to_string()));
        assert_eq!(record.ttl, Some(3600));
    }

    #[test]
    fn add_record_mx_with_priority() {
        use crate::types::RecordType;

        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"add-record","params":{"domain":"example.com","type":"MX","name":"@","content":"mail.example.com","ttl":3600,"prio":10}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "rec124",
                        "name": "@",
                        "type": "MX",
                        "content": "mail.example.com",
                        "ttl": 3600,
                        "prio": 10
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let params = AddRecordParams {
            domain: "example.com".to_string(),
            record_type: RecordType::Mx,
            name: "@".to_string(),
            content: Some("mail.example.com".to_string()),
            ttl: Some(3600),
            priority: Some(10),
            weight: None,
            port: None,
            target: None,
            value: None,
            ssh_algorithm: None,
            ssh_type: None,
        };
        let record = client.add_record(&params).unwrap();

        assert_eq!(record.id, "rec124");
        assert_eq!(record.record_type, RecordType::Mx);
        assert_eq!(record.priority, Some(10));
    }

    #[test]
    fn add_record_srv_full() {
        use crate::types::RecordType;

        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"add-record","params":{"domain":"example.com","type":"SRV","name":"_sip._tcp","content":"sipserver.example.com","ttl":3600,"prio":10,"weight":5,"port":5060}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "rec125",
                        "name": "_sip._tcp",
                        "type": "SRV",
                        "content": "sipserver.example.com",
                        "ttl": 3600,
                        "prio": 10,
                        "weight": 5,
                        "port": 5060
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let params = AddRecordParams {
            domain: "example.com".to_string(),
            record_type: RecordType::Srv,
            name: "_sip._tcp".to_string(),
            content: Some("sipserver.example.com".to_string()),
            ttl: Some(3600),
            priority: Some(10),
            weight: Some(5),
            port: Some(5060),
            target: None,
            value: None,
            ssh_algorithm: None,
            ssh_type: None,
        };
        let record = client.add_record(&params).unwrap();

        assert_eq!(record.id, "rec125");
        assert_eq!(record.record_type, RecordType::Srv);
        assert_eq!(record.weight, Some(5));
        assert_eq!(record.port, Some(5060));
    }

    #[test]
    fn add_record_dynamic() {
        use crate::types::RecordType;

        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"add-record","params":{"domain":"example.com","type":"Dynamic","name":"home"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "rec126",
                        "name": "home",
                        "type": "Dynamic"
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let params = AddRecordParams {
            domain: "example.com".to_string(),
            record_type: RecordType::Dynamic,
            name: "home".to_string(),
            content: None,
            ttl: None,
            priority: None,
            weight: None,
            port: None,
            target: None,
            value: None,
            ssh_algorithm: None,
            ssh_type: None,
        };
        let record = client.add_record(&params).unwrap();

        assert_eq!(record.id, "rec126");
        assert_eq!(record.record_type, RecordType::Dynamic);
    }

    #[test]
    fn edit_record_updates_content() {
        use crate::types::RecordType;

        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"edit-record","params":{"domain":"example.com","id":"rec123","content":"5.6.7.8","ttl":300}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {
                        "id": "rec123",
                        "name": "@",
                        "type": "A",
                        "content": "5.6.7.8",
                        "ttl": 300
                    }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let params = EditRecordParams {
            domain: "example.com".to_string(),
            id: "rec123".to_string(),
            name: None,
            content: Some("5.6.7.8".to_string()),
            ttl: Some(300),
            priority: None,
            weight: None,
            port: None,
            target: None,
            value: None,
            ssh_algorithm: None,
            ssh_type: None,
        };
        let record = client.edit_record(&params).unwrap();

        assert_eq!(record.id, "rec123");
        assert_eq!(record.record_type, RecordType::A);
        assert_eq!(record.content, Some("5.6.7.8".to_string()));
        assert_eq!(record.ttl, Some(300));
    }

    #[test]
    fn remove_record_deletes_record() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"remove-record","params":{"domain":"example.com","id":"rec123"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "result": {}
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let result = client.remove_record("example.com", "rec123");

        assert!(result.is_ok());
    }

    #[test]
    fn remove_record_not_found() {
        let mock_server = mock_server();

        mount(
            &mock_server,
            Mock::given(method("POST"))
                .and(body_json_string(
                    r#"{"method":"remove-record","params":{"domain":"example.com","id":"notfound"}}"#,
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "error": { "message": "Record not found" }
                })))
                .expect(1),
        );

        let client = NjallaClient::with_base_url("token", &mock_server.uri());
        let result = client.remove_record("example.com", "notfound");

        assert!(matches!(result, Err(NjallaError::Api { message }) if message == "Record not found"));
    }
}
