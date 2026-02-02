//! Output formatting for CLI commands.

use crate::error::Result;
use crate::types::{Domain, MarketDomain, Payment, Record, Transaction, ValidationResult, WalletBalance};

/// Format a list of domains for output.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_domains(domains: &[Domain]) -> Result<String> {
    Ok(serde_json::to_string_pretty(domains)?)
}

/// Format market domain search results.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_market_domains(domains: &[MarketDomain]) -> Result<String> {
    Ok(serde_json::to_string_pretty(domains)?)
}

/// Format a single domain status.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_domain_status(
    domain: &Domain,
    records: Option<&[Record]>,
) -> Result<String> {
    let result = serde_json::json!({
        "domain": domain,
        "dns_records": records,
    });
    Ok(serde_json::to_string_pretty(&result)?)
}

/// Format validation results.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
#[allow(dead_code)]
pub fn format_validation(result: &ValidationResult) -> Result<String> {
    Ok(serde_json::to_string_pretty(result)?)
}

/// Format wallet balance.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_wallet_balance(balance: &WalletBalance) -> Result<String> {
    Ok(serde_json::to_string_pretty(balance)?)
}

/// Format payment information.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_payment(payment: &Payment) -> Result<String> {
    Ok(serde_json::to_string_pretty(payment)?)
}

/// Format a list of transactions.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_transactions(transactions: &[Transaction]) -> Result<String> {
    Ok(serde_json::to_string_pretty(transactions)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_empty_domains() {
        let result = format_domains(&[]).unwrap();
        assert_eq!(result, "[]");
    }

    #[test]
    fn format_wallet_balance_json() {
        let balance = WalletBalance { balance: 150 };
        let result = format_wallet_balance(&balance).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["balance"], 150);
    }

    #[test]
    fn format_payment_json() {
        let payment = Payment {
            id: Some("pay123".to_string()),
            amount: 30,
            currency: Some("EUR".to_string()),
            amount_btc: Some("0.0005128".to_string()),
            status: Some("Waiting for transaction".to_string()),
            address: Some("bc1qtest".to_string()),
            uri: Some("bitcoin:bc1qtest?amount=0.0005128".to_string()),
            url: None,
        };
        let result = format_payment(&payment).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["id"], "pay123");
        assert_eq!(parsed["amount"], 30);
        assert_eq!(parsed["currency"], "EUR");
        assert_eq!(parsed["amount_btc"], "0.0005128");
        assert_eq!(parsed["address"], "bc1qtest");
        assert_eq!(parsed["uri"], "bitcoin:bc1qtest?amount=0.0005128");
    }

    #[test]
    fn format_transactions_empty() {
        let result = format_transactions(&[]).unwrap();
        assert_eq!(result, "[]");
    }

    #[test]
    fn format_transactions_json() {
        let transactions = vec![Transaction {
            id: "tx1".to_string(),
            amount: 50,
            status: "Added 50 € via Bitcoin".to_string(),
            completed: Some("2026-01-15".to_string()),
            pdf: None,
            uri: None,
            address: None,
            currency: None,
            amount_btc: None,
        }];
        let result = format_transactions(&transactions).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed[0]["id"], "tx1");
    }
}
