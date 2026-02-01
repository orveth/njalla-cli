//! API types for Njalla.
//!
//! These types map directly to the Njalla API JSON structures.
//! See `docs/API.md` for full API documentation.

use chrono::{DateTime, Utc};
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

    /// Expiration date.
    #[serde(default)]
    pub expiry: Option<DateTime<Utc>>,

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

/// DNS record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    /// Record ID.
    pub id: String,

    /// Record name (e.g., "@", "www").
    pub name: String,

    /// Record type (e.g., "A", "AAAA", "CNAME").
    #[serde(rename = "type")]
    #[allow(clippy::struct_field_names)]
    pub record_type: String,

    /// Record content/value.
    pub content: String,

    /// Time to live in seconds.
    pub ttl: i32,

    /// Priority (for MX, SRV records).
    #[serde(rename = "prio")]
    pub priority: Option<i32>,
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
// Response Result Types
// ============================================================================

/// Response for `list-domains`.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DomainsResult {
    /// List of domains.
    pub domains: Vec<Domain>,
}

/// Response for `find-domains`.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MarketDomainsResult {
    /// List of domain search results.
    pub domains: Vec<MarketDomain>,
}

/// Response for `list-records`.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RecordsResult {
    /// List of DNS records.
    pub records: Vec<Record>,
}

/// Response for `register-domain`.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RegisterResult {
    /// Task ID for tracking registration.
    pub task: String,
}

// ============================================================================
// Validation Types
// ============================================================================

/// Result of domain registration validation.
#[derive(Debug, Serialize)]
#[allow(dead_code, clippy::struct_excessive_bools)]
pub struct ValidationResult {
    /// Domain being validated.
    pub domain: String,

    /// Overall validation passed.
    pub valid: bool,

    /// Individual check results.
    pub checks: ValidationChecks,

    /// Domain info if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_info: Option<Domain>,

    /// DNS records if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<Vec<Record>>,

    /// Error message if validation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Individual validation checks.
#[derive(Debug, Serialize)]
#[allow(dead_code, clippy::struct_excessive_bools)]
pub struct ValidationChecks {
    /// Domain exists in account.
    pub exists: bool,

    /// Domain status is "active".
    pub status_active: bool,

    /// Domain has an expiry date.
    pub has_expiry: bool,

    /// DNS records are accessible.
    pub dns_accessible: bool,
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
        assert_eq!(record.record_type, "A");
        assert_eq!(record.ttl, 10800);
        assert!(record.priority.is_none());
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
}
