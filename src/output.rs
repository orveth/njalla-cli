//! Output formatting for CLI commands.

use crate::error::Result;
use crate::types::{Domain, MarketDomain, Record, ValidationResult};
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
}
