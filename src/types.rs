//! API types for Njalla.
//!
//! These types map directly to the Njalla API JSON structures.
//! See `docs/API.md` for full API documentation.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

// ============================================================================
// Domain Types
// ============================================================================

/// Domain information returned by `list-domains` and `get-domain`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    /// Domain name (e.g., "example.com").
    pub name: String,

    /// Domain status (e.g., "active", "pending").
    pub status: String,

    /// Expiration date (ISO 8601 format).
    #[serde(default)]
    pub expiry: Option<String>,

    /// Whether the domain is locked for transfer.
    #[serde(default)]
    pub locked: Option<bool>,

    /// Whether mail forwarding is enabled.
    #[serde(default)]
    pub mailforwarding: Option<bool>,

    /// Maximum number of nameservers allowed.
    #[serde(default)]
    pub max_nameservers: Option<i32>,
}

/// Domain availability and pricing from `find-domains`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDomain {
    /// Domain name.
    pub name: String,

    /// Availability status ("available" or "taken").
    pub status: String,

    /// Price in EUR per year.
    pub price: i32,
}

// ============================================================================
// DNS Types
// ============================================================================

/// DNS record type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum RecordType {
    A,
    #[serde(rename = "AAAA")]
    Aaaa,
    #[serde(rename = "ANAME")]
    Aname,
    #[serde(rename = "CAA")]
    Caa,
    #[serde(rename = "CNAME")]
    Cname,
    #[serde(rename = "DS")]
    Ds,
    Dynamic,
    #[serde(rename = "HTTPS")]
    Https,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "NAPTR")]
    Naptr,
    #[serde(rename = "NS")]
    Ns,
    #[serde(rename = "PTR")]
    Ptr,
    #[serde(rename = "SRV")]
    Srv,
    #[serde(rename = "SSHFP")]
    Sshfp,
    #[serde(rename = "SVCB")]
    Svcb,
    #[serde(rename = "TLSA")]
    Tlsa,
    #[serde(rename = "TXT")]
    Txt,
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::A => "A",
            Self::Aaaa => "AAAA",
            Self::Aname => "ANAME",
            Self::Caa => "CAA",
            Self::Cname => "CNAME",
            Self::Ds => "DS",
            Self::Dynamic => "Dynamic",
            Self::Https => "HTTPS",
            Self::Mx => "MX",
            Self::Naptr => "NAPTR",
            Self::Ns => "NS",
            Self::Ptr => "PTR",
            Self::Srv => "SRV",
            Self::Sshfp => "SSHFP",
            Self::Svcb => "SVCB",
            Self::Tlsa => "TLSA",
            Self::Txt => "TXT",
        };
        write!(f, "{s}")
    }
}

/// DNS record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    /// Record ID.
    pub id: String,

    /// Record name (e.g., "@", "www").
    pub name: String,

    /// Record type (e.g., A, AAAA, CNAME).
    #[serde(rename = "type")]
    #[allow(clippy::struct_field_names)]
    pub record_type: RecordType,

    /// Record content/value.
    #[serde(default)]
    pub content: Option<String>,

    /// Time to live in seconds.
    #[serde(default)]
    pub ttl: Option<i32>,

    /// Priority (for MX, SRV, HTTPS, SVCB records).
    #[serde(rename = "prio", default)]
    pub priority: Option<i32>,

    /// Weight (SRV records only).
    #[serde(default)]
    pub weight: Option<i32>,

    /// Port (SRV records only).
    #[serde(default)]
    pub port: Option<i32>,

    /// Target (HTTPS, SVCB records only).
    #[serde(default)]
    pub target: Option<String>,

    /// Value/SvcParams (HTTPS, SVCB records only).
    #[serde(default)]
    pub value: Option<String>,

    /// SSH algorithm (SSHFP records only, 1-5: RSA, DSA, ECDSA, Ed25519, XMSS).
    #[serde(default)]
    pub ssh_algorithm: Option<i32>,

    /// SSH fingerprint type (SSHFP records only, 1-2: SHA-1, SHA-256).
    #[serde(default)]
    pub ssh_type: Option<i32>,
}

/// Parameters for adding a DNS record.
#[derive(Debug, Clone)]
pub struct AddRecordParams {
    /// Domain name.
    pub domain: String,
    /// Record type.
    pub record_type: RecordType,
    /// Record name (e.g., "@", "www").
    pub name: String,
    /// Record content/value.
    pub content: Option<String>,
    /// TTL in seconds.
    pub ttl: Option<i32>,
    /// Priority (MX, SRV, HTTPS, SVCB).
    pub priority: Option<i32>,
    /// Weight (SRV only).
    pub weight: Option<i32>,
    /// Port (SRV only).
    pub port: Option<i32>,
    /// Target (HTTPS, SVCB only).
    pub target: Option<String>,
    /// Value/SvcParams (HTTPS, SVCB only).
    pub value: Option<String>,
    /// SSH algorithm (SSHFP only, 1-5).
    pub ssh_algorithm: Option<i32>,
    /// SSH fingerprint type (SSHFP only, 1-2).
    pub ssh_type: Option<i32>,
}

/// Parameters for editing a DNS record.
#[derive(Debug, Clone)]
pub struct EditRecordParams {
    /// Domain name.
    pub domain: String,
    /// Record ID.
    pub id: String,
    /// Record name (e.g., "@", "www").
    pub name: Option<String>,
    /// Record content/value.
    pub content: Option<String>,
    /// TTL in seconds.
    pub ttl: Option<i32>,
    /// Priority (MX, SRV, HTTPS, SVCB).
    pub priority: Option<i32>,
    /// Weight (SRV only).
    pub weight: Option<i32>,
    /// Port (SRV only).
    pub port: Option<i32>,
    /// Target (HTTPS, SVCB only).
    pub target: Option<String>,
    /// Value/SvcParams (HTTPS, SVCB only).
    pub value: Option<String>,
    /// SSH algorithm (SSHFP only, 1-5).
    pub ssh_algorithm: Option<i32>,
    /// SSH fingerprint type (SSHFP only, 1-2).
    pub ssh_type: Option<i32>,
}

// ============================================================================
// Task Types
// ============================================================================

/// Task status for async operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    /// Task ID.
    pub id: String,

    /// Task status ("pending", "processing", "completed", "failed").
    pub status: String,
}

// ============================================================================
// API Request/Response Types
// ============================================================================

/// API request body (JSON-RPC style).
#[derive(Debug, Serialize)]
pub struct ApiRequest {
    /// API method name.
    pub method: String,

    /// Method parameters.
    pub params: serde_json::Value,
}

/// API response wrapper.
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// Successful result.
    pub result: Option<T>,

    /// Error information.
    #[serde(default)]
    pub error: Option<ApiError>,
}

/// API error response.
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// Error message.
    pub message: String,
}

// ============================================================================
// Wallet Types
// ============================================================================

/// Wallet balance information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Current balance in euros.
    pub balance: i32,
}

/// Payment information from `add-payment` or `get-payment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// Payment ID.
    #[serde(default)]
    pub id: Option<String>,

    /// Payment amount in euros.
    pub amount: i32,

    /// Currency (e.g., "EUR").
    #[serde(default)]
    pub currency: Option<String>,

    /// Amount in BTC (for Bitcoin payments).
    #[serde(default)]
    pub amount_btc: Option<String>,

    /// Payment status (for get-payment).
    #[serde(default)]
    pub status: Option<String>,

    /// Payment address (for crypto payments).
    #[serde(default)]
    pub address: Option<String>,

    /// Bitcoin URI for payment (e.g., "bitcoin:address?amount=X").
    #[serde(default)]
    pub uri: Option<String>,

    /// Payment URL (if provided by the API).
    #[serde(default)]
    pub url: Option<String>,
}

/// A wallet transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID.
    pub id: String,

    /// Transaction amount in euros.
    pub amount: i32,

    /// Transaction status/description.
    pub status: String,

    /// Completion date (for completed transactions).
    #[serde(default)]
    pub completed: Option<String>,

    /// Invoice PDF URL (for completed transactions).
    #[serde(default)]
    pub pdf: Option<String>,

    /// Bitcoin URI (for pending payments).
    #[serde(default)]
    pub uri: Option<String>,

    /// Payment address (for pending crypto payments).
    #[serde(default)]
    pub address: Option<String>,

    /// Currency code (for pending payments).
    #[serde(default)]
    pub currency: Option<String>,

    /// Amount in BTC (for Bitcoin payments).
    #[serde(default)]
    pub amount_btc: Option<String>,
}

/// Response for `list-transactions`.
#[derive(Debug, Deserialize)]
pub struct TransactionsResult {
    /// List of transactions.
    pub transactions: Vec<Transaction>,
}

/// Payment method for wallet top-up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum PaymentMethod {
    #[value(alias = "btc")]
    Bitcoin,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bitcoin => write!(f, "bitcoin"),
        }
    }
}

// ============================================================================
// Response Result Types
// ============================================================================

/// Response for `list-domains`.
#[derive(Debug, Deserialize)]
pub struct DomainsResult {
    /// List of domains.
    pub domains: Vec<Domain>,
}

/// Response for `find-domains`.
#[derive(Debug, Deserialize)]
pub struct MarketDomainsResult {
    /// List of domain search results.
    pub domains: Vec<MarketDomain>,
}

/// Response for `list-records`.
#[derive(Debug, Deserialize)]
pub struct RecordsResult {
    /// List of DNS records.
    pub records: Vec<Record>,
}

/// Response for `register-domain`.
#[derive(Debug, Deserialize)]
pub struct RegisterResult {
    /// Task ID for tracking registration.
    pub task: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_domain() {
        let json = r#"{
            "name": "example.com",
            "status": "active",
            "expiry": "2027-01-15T00:00:00Z",
            "locked": false
        }"#;

        let domain: Domain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.status, "active");
        assert_eq!(domain.locked, Some(false));
    }

    #[test]
    fn deserialize_market_domain() {
        let json = r#"{
            "name": "example.com",
            "status": "available",
            "price": 15
        }"#;

        let domain: MarketDomain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.status, "available");
        assert_eq!(domain.price, 15);
    }

    #[test]
    fn deserialize_record() {
        let json = r#"{
            "id": "12345",
            "name": "@",
            "type": "A",
            "content": "192.0.2.1",
            "ttl": 10800,
            "prio": null
        }"#;

        let record: Record = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "12345");
        assert_eq!(record.record_type, RecordType::A);
        assert_eq!(record.ttl, Some(10800));
        assert!(record.priority.is_none());
    }

    #[test]
    fn deserialize_record_srv() {
        let json = r#"{
            "id": "12346",
            "name": "_sip._tcp",
            "type": "SRV",
            "content": "sipserver.example.com",
            "ttl": 3600,
            "prio": 10,
            "weight": 5,
            "port": 5060
        }"#;

        let record: Record = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "12346");
        assert_eq!(record.record_type, RecordType::Srv);
        assert_eq!(record.priority, Some(10));
        assert_eq!(record.weight, Some(5));
        assert_eq!(record.port, Some(5060));
    }

    #[test]
    fn deserialize_record_dynamic() {
        let json = r#"{
            "id": "12347",
            "name": "home",
            "type": "Dynamic"
        }"#;

        let record: Record = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "12347");
        assert_eq!(record.record_type, RecordType::Dynamic);
        assert!(record.content.is_none());
        assert!(record.ttl.is_none());
    }

    #[test]
    fn record_type_display() {
        assert_eq!(RecordType::A.to_string(), "A");
        assert_eq!(RecordType::Aaaa.to_string(), "AAAA");
        assert_eq!(RecordType::Mx.to_string(), "MX");
        assert_eq!(RecordType::Dynamic.to_string(), "Dynamic");
        assert_eq!(RecordType::Sshfp.to_string(), "SSHFP");
    }

    #[test]
    fn record_type_serialize() {
        assert_eq!(
            serde_json::to_string(&RecordType::A).unwrap(),
            "\"A\""
        );
        assert_eq!(
            serde_json::to_string(&RecordType::Aaaa).unwrap(),
            "\"AAAA\""
        );
        assert_eq!(
            serde_json::to_string(&RecordType::Dynamic).unwrap(),
            "\"Dynamic\""
        );
    }

    #[test]
    fn serialize_api_request() {
        let req = ApiRequest {
            method: "list-domains".to_string(),
            params: serde_json::json!({}),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("list-domains"));
    }

    #[test]
    fn payment_method_display() {
        assert_eq!(PaymentMethod::Bitcoin.to_string(), "bitcoin");
    }

    #[test]
    fn deserialize_wallet_balance() {
        let json = r#"{"balance": 100}"#;
        let balance: WalletBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.balance, 100);
    }

    #[test]
    fn deserialize_payment() {
        let json = r#"{
            "id": "pay123",
            "amount": 15,
            "currency": "EUR",
            "amount_btc": "0.0002564",
            "status": "Waiting for transaction of 15 € via Bitcoin to be confirmed",
            "address": "bc1qtest",
            "uri": "bitcoin:bc1qtest?amount=0.0002564"
        }"#;
        let payment: Payment = serde_json::from_str(json).unwrap();
        assert_eq!(payment.id, Some("pay123".to_string()));
        assert_eq!(payment.amount, 15);
        assert_eq!(payment.currency, Some("EUR".to_string()));
        assert_eq!(payment.amount_btc, Some("0.0002564".to_string()));
        assert_eq!(payment.status, Some("Waiting for transaction of 15 € via Bitcoin to be confirmed".to_string()));
        assert_eq!(payment.address, Some("bc1qtest".to_string()));
        assert_eq!(payment.uri, Some("bitcoin:bc1qtest?amount=0.0002564".to_string()));
        assert!(payment.url.is_none());
    }

    #[test]
    fn deserialize_transaction_completed() {
        let json = r#"{
            "id": "IKSELBVIY5JW4UAER7PGLFEPSGHOJNB7",
            "amount": 210,
            "status": "Added 210 € via Bitcoin",
            "completed": "2026-02-01",
            "pdf": "https://njal.la/invoice/IKSELBVIY5JW4UAER7PGLFEPSGHOJNB7/"
        }"#;
        let tx: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(tx.id, "IKSELBVIY5JW4UAER7PGLFEPSGHOJNB7");
        assert_eq!(tx.amount, 210);
        assert_eq!(tx.status, "Added 210 € via Bitcoin");
        assert_eq!(tx.completed, Some("2026-02-01".to_string()));
        assert!(tx.pdf.is_some());
    }

    #[test]
    fn deserialize_transaction_pending() {
        let json = r#"{
            "id": "4S4IQTHCP3URAUMYUXCY4UTUGU666CVK",
            "amount": 15,
            "status": "Waiting for transaction of 15 € via Bitcoin to be confirmed",
            "uri": "bitcoin:bc1qtest?amount=0.0002539",
            "address": "bc1qtest",
            "currency": "EUR",
            "amount_btc": "0.0002539"
        }"#;
        let tx: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(tx.id, "4S4IQTHCP3URAUMYUXCY4UTUGU666CVK");
        assert_eq!(tx.amount, 15);
        assert!(tx.uri.is_some());
        assert!(tx.completed.is_none());
    }
}
