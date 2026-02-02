//! Output formatting for CLI commands.

use crate::error::Result;
use crate::types::{Domain, MarketDomain, Payment, Record, Transaction, ValidationResult, WalletBalance};
use colored::Colorize;

/// Output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable table format.
    Table,
    /// JSON format for scripting.
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "json" => Self::Json,
            _ => Self::Table,
        })
    }
}

/// Format a domain status with appropriate colors.
fn format_status(status: &str) -> String {
    match status {
        "active" => status.green().to_string(),
        "pending" => status.yellow().to_string(),
        _ => status.red().to_string(),
    }
}

/// Format a list of domains for output.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
#[allow(clippy::format_push_string)]
pub fn format_domains(domains: &[Domain], format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(domains)?),
        OutputFormat::Table => {
            if domains.is_empty() {
                return Ok("No domains found".to_string());
            }

            let mut output = String::new();
            output.push_str(&format!(
                "{:<35} {:<12} {:<25}\n",
                "DOMAIN".bold(),
                "STATUS".bold(),
                "EXPIRY".bold()
            ));
            output.push_str(&"-".repeat(75));
            output.push('\n');

            for d in domains {
                let status = format_status(&d.status);

                let expiry = d
                    .expiry
                    .map_or_else(|| "-".to_string(), |e| e.format("%Y-%m-%d").to_string());

                output.push_str(&format!("{:<35} {:<12} {:<25}\n", d.name, status, expiry));
            }

            Ok(output)
        }
    }
}

/// Format market domain search results.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
#[allow(clippy::format_push_string)]
pub fn format_market_domains(domains: &[MarketDomain], format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(domains)?),
        OutputFormat::Table => {
            if domains.is_empty() {
                return Ok("No results found".to_string());
            }

            let mut output = String::new();
            output.push_str(&format!(
                "{:<40} {:<12} {:<10}\n",
                "DOMAIN".bold(),
                "AVAILABLE".bold(),
                "PRICE".bold()
            ));
            output.push_str(&"-".repeat(65));
            output.push('\n');

            let mut available_count = 0;
            for d in domains {
                let (icon, status_str) = if d.status == "available" {
                    available_count += 1;
                    ("✓".green().to_string(), "available")
                } else {
                    ("✗".red().to_string(), "taken")
                };

                output.push_str(&format!(
                    "{:<40} {} {:<10} €{}/yr\n",
                    d.name, icon, status_str, d.price
                ));
            }

            output.push('\n');
            output.push_str(&format!(
                "{} of {} domains available\n",
                available_count.to_string().green(),
                domains.len()
            ));

            Ok(output)
        }
    }
}

/// Format a single domain status.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
#[allow(clippy::format_push_string)]
pub fn format_domain_status(
    domain: &Domain,
    records: Option<&[Record]>,
    format: OutputFormat,
) -> Result<String> {
    match format {
        OutputFormat::Json => {
            let result = serde_json::json!({
                "domain": domain,
                "dns_records": records,
            });
            Ok(serde_json::to_string_pretty(&result)?)
        }
        OutputFormat::Table => {
            let mut output = String::new();

            let status = format_status(&domain.status);

            output.push_str(&format!("{}: {}\n", "Domain".bold(), domain.name.cyan()));
            output.push_str(&format!("{}: {}\n", "Status".bold(), status));

            if let Some(expiry) = domain.expiry {
                output.push_str(&format!(
                    "{}: {}\n",
                    "Expiry".bold(),
                    expiry.format("%Y-%m-%d")
                ));
            }

            if let Some(locked) = domain.locked {
                output.push_str(&format!("{}: {}\n", "Locked".bold(), locked));
            }

            if let Some(records) = records {
                output.push('\n');
                output.push_str(&format!("{} ({}):\n", "DNS Records".bold(), records.len()));
                output.push_str(&format!(
                    "  {:<20} {:<8} {:<40} {:<6}\n",
                    "NAME", "TYPE", "CONTENT", "TTL"
                ));
                output.push_str(&format!("  {}\n", "-".repeat(76)));

                for r in records {
                    output.push_str(&format!(
                        "  {:<20} {:<8} {:<40} {:<6}\n",
                        r.name, r.record_type, r.content, r.ttl
                    ));
                }
            }

            Ok(output)
        }
    }
}

/// Format validation results.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
#[allow(clippy::format_push_string, dead_code)]
pub fn format_validation(result: &ValidationResult, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(result)?),
        OutputFormat::Table => {
            let mut output = String::new();

            output.push_str(&format!("{}\n", "Validation Results".bold()));
            output.push_str(&format!("{}: {}\n\n", "Domain".bold(), result.domain.cyan()));

            let check_line = |name: &str, passed: bool| -> String {
                let (icon, status) = if passed {
                    ("✓".green().to_string(), "passed".green().to_string())
                } else {
                    ("✗".red().to_string(), "failed".red().to_string())
                };
                format!("  {icon} {name} - {status}\n")
            };

            output.push_str(&check_line("Domain exists", result.checks.exists));
            output.push_str(&check_line("Status is active", result.checks.status_active));
            output.push_str(&check_line("Has expiry date", result.checks.has_expiry));
            output.push_str(&check_line("DNS accessible", result.checks.dns_accessible));

            output.push('\n');

            if result.valid {
                output.push_str(&format!(
                    "{} Domain {} is properly registered!\n",
                    "✓".green().bold(),
                    result.domain.cyan()
                ));
            } else {
                output.push_str(&format!(
                    "{} Validation failed for {}\n",
                    "✗".red().bold(),
                    result.domain.cyan()
                ));
                if let Some(ref error) = result.error {
                    output.push_str(&format!("  Error: {error}\n"));
                }
            }

            Ok(output)
        }
    }
}

/// Format wallet balance.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
pub fn format_wallet_balance(balance: &WalletBalance, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(balance)?),
        OutputFormat::Table => {
            let balance_str = format!("{}{}",  "€".bold(), balance.balance.to_string().green().bold());
            Ok(format!("{}: {}\n", "Wallet Balance".bold(), balance_str))
        }
    }
}

/// Format payment information.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
#[allow(clippy::format_push_string)]
pub fn format_payment(payment: &Payment, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(payment)?),
        OutputFormat::Table => {
            let mut output = String::new();

            output.push_str(&format!("{}\n", "Payment Details".bold()));
            output.push_str(&format!("{}\n", "-".repeat(40)));

            if let Some(ref id) = payment.id {
                output.push_str(&format!("{}: {}\n", "ID".bold(), id));
            }

            // Show amount with currency, and BTC amount if available
            let currency = payment.currency.as_deref().unwrap_or("EUR");
            let currency_symbol = if currency == "EUR" { "€" } else { currency };
            output.push_str(&format!("{}: {}{}\n", "Amount".bold(), currency_symbol, payment.amount));

            if let Some(ref amount_btc) = payment.amount_btc {
                output.push_str(&format!("{}: {} BTC\n", "Amount (BTC)".bold(), amount_btc.cyan()));
            }

            if let Some(ref status) = payment.status {
                let status_colored = if status.contains("Waiting") || status.contains("pending") {
                    status.yellow().to_string()
                } else if status.contains("completed") || status.contains("paid") || status.contains("confirmed") {
                    status.green().to_string()
                } else {
                    status.clone()
                };
                output.push_str(&format!("{}: {}\n", "Status".bold(), status_colored));
            }

            if let Some(ref address) = payment.address {
                output.push_str(&format!("{}: {}\n", "Address".bold(), address.cyan()));
            }

            if let Some(ref uri) = payment.uri {
                output.push_str(&format!("{}: {}\n", "URI".bold(), uri.cyan()));
            }

            if let Some(ref url) = payment.url {
                output.push_str(&format!("{}: {}\n", "URL".bold(), url.cyan()));
            }

            Ok(output)
        }
    }
}

/// Format a list of transactions.
///
/// # Errors
///
/// Returns an error if JSON serialization fails.
#[allow(clippy::format_push_string)]
pub fn format_transactions(transactions: &[Transaction], format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(transactions)?),
        OutputFormat::Table => {
            if transactions.is_empty() {
                return Ok("No transactions found\n".to_string());
            }

            let mut output = String::new();
            output.push_str(&format!(
                "{:<12} {:<10} {:<14} {:<45} {}\n",
                "DATE".bold(),
                "AMOUNT".bold(),
                "BTC".bold(),
                "ADDRESS".bold(),
                "STATUS".bold()
            ));
            output.push_str(&"-".repeat(120));
            output.push('\n');

            for t in transactions {
                let date = t.completed.as_deref().unwrap_or("pending");

                // Pad before coloring to avoid ANSI codes breaking alignment
                let amount_raw = format!("€{}", t.amount);
                let amount_padded = format!("{amount_raw:<10}");
                let amount_str = if t.completed.is_some() {
                    amount_padded.green().to_string()
                } else {
                    amount_padded.yellow().to_string()
                };

                let btc_amount = t.amount_btc.as_deref().unwrap_or("-");
                let address = t.address.as_deref().unwrap_or("-");

                output.push_str(&format!(
                    "{:<12} {} {:<14} {:<45} {}\n",
                    date, amount_str, btc_amount, address, t.status
                ));
            }

            output.push('\n');
            output.push_str(&format!("{} transactions\n", transactions.len()));

            Ok(output)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_format_from_str() {
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("JSON".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("table".parse::<OutputFormat>().unwrap(), OutputFormat::Table);
        assert_eq!("anything".parse::<OutputFormat>().unwrap(), OutputFormat::Table);
    }

    #[test]
    fn format_empty_domains() {
        let result = format_domains(&[], OutputFormat::Table).unwrap();
        assert_eq!(result, "No domains found");
    }

    #[test]
    fn format_empty_domains_json() {
        let result = format_domains(&[], OutputFormat::Json).unwrap();
        assert_eq!(result, "[]");
    }

    #[test]
    fn format_wallet_balance_table() {
        let balance = WalletBalance { balance: 150 };
        let result = format_wallet_balance(&balance, OutputFormat::Table).unwrap();
        assert!(result.contains("150"));
        assert!(result.contains("Wallet Balance"));
    }

    #[test]
    fn format_wallet_balance_json() {
        let balance = WalletBalance { balance: 150 };
        let result = format_wallet_balance(&balance, OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["balance"], 150);
    }

    #[test]
    fn format_payment_table() {
        let payment = Payment {
            id: Some("pay123".to_string()),
            amount: 30,
            currency: Some("EUR".to_string()),
            amount_btc: Some("0.0005128".to_string()),
            status: Some("Waiting for transaction of 30 € via Bitcoin to be confirmed".to_string()),
            address: Some("bc1qtest".to_string()),
            uri: Some("bitcoin:bc1qtest?amount=0.0005128".to_string()),
            url: None,
        };
        let result = format_payment(&payment, OutputFormat::Table).unwrap();
        assert!(result.contains("pay123"));
        assert!(result.contains("30"));
        assert!(result.contains("Waiting"));
        assert!(result.contains("bc1qtest"));
        assert!(result.contains("0.0005128"));
        assert!(result.contains("Amount (BTC)"));
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
        let result = format_payment(&payment, OutputFormat::Json).unwrap();
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
        let result = format_transactions(&[], OutputFormat::Table).unwrap();
        assert!(result.contains("No transactions found"));
    }

    #[test]
    fn format_transactions_table() {
        let transactions = vec![
            Transaction {
                id: "tx1".to_string(),
                amount: 210,
                status: "Added 210 € via Bitcoin".to_string(),
                completed: Some("2026-02-01".to_string()),
                pdf: Some("https://njal.la/invoice/tx1/".to_string()),
                uri: None,
                address: None,
                currency: None,
                amount_btc: None,
            },
            Transaction {
                id: "tx2".to_string(),
                amount: 15,
                status: "Waiting for transaction".to_string(),
                completed: None,
                pdf: None,
                uri: Some("bitcoin:bc1qtest".to_string()),
                address: Some("bc1qtest".to_string()),
                currency: Some("EUR".to_string()),
                amount_btc: Some("0.0002539".to_string()),
            },
        ];
        let result = format_transactions(&transactions, OutputFormat::Table).unwrap();
        assert!(result.contains("Added 210"));
        assert!(result.contains("Waiting for"));
        assert!(result.contains("2 transactions"));
        // Check new columns
        assert!(result.contains("BTC"));
        assert!(result.contains("ADDRESS"));
        assert!(result.contains("bc1qtest"));
        assert!(result.contains("0.0002539"));
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
        let result = format_transactions(&transactions, OutputFormat::Json).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed[0]["id"], "tx1");
    }
}
