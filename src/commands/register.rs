//! Register domain command.

use crate::client::NjallaClient;
use crate::error::{NjallaError, Result};
use crate::output::OutputFormat;
use colored::Colorize;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

/// Poll interval for checking task status.
const POLL_INTERVAL_SECS: u64 = 2;

/// Run the register command.
///
/// Registers a new domain through Njalla.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub async fn run(
    domain: &str,
    years: i32,
    confirm: bool,
    wait: bool,
    timeout: u64,
    output: &str,
    debug: bool,
) -> Result<()> {
    let client = NjallaClient::new(debug)?;
    let format: OutputFormat = output.parse().expect("infallible");

    // Check domain availability and get price
    let search_results = client.find_domains(domain).await?;
    let domain_info = search_results.iter().find(|d| d.name == domain);

    let Some(info) = domain_info else {
        return Err(NjallaError::DomainNotAvailable(format!(
            "{domain} not found in search results"
        )));
    };

    if info.status != "available" {
        let reason = match info.status.as_str() {
            "taken" => format!("{domain} is already registered"),
            "in progress" => format!("{domain} registration is already in progress"),
            "failed" => format!("{domain} registration previously failed"),
            _ => format!("{domain} is not available (status: {})", info.status),
        };
        return Err(NjallaError::DomainNotAvailable(reason));
    }

    let total_price = info.price * years;

    // Show confirmation unless --confirm flag is set
    if !confirm {
        println!(
            "Domain: {}\nPrice: {} EUR/year\nYears: {}\nTotal: {} EUR\n",
            domain.cyan(),
            info.price,
            years,
            total_price.to_string().green()
        );
        print!("Proceed with registration? [y/N] ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Registration cancelled.");
            return Ok(());
        }
    }

    // Register the domain
    let task_id = client.register_domain(domain, years).await?;

    if !wait {
        // Output task ID and exit
        match format {
            OutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "domain": domain,
                        "task_id": task_id,
                        "status": "pending"
                    }))?
                );
            }
            OutputFormat::Table => {
                println!(
                    "Registration started for {}\nTask ID: {}\n\nUse 'njalla status {}' to check progress.",
                    domain.cyan(),
                    task_id.yellow(),
                    domain
                );
            }
        }
        return Ok(());
    }

    // Poll for completion
    println!("Waiting for registration to complete...");
    let start = std::time::Instant::now();
    let timeout_duration = Duration::from_secs(timeout);

    loop {
        if start.elapsed() > timeout_duration {
            return Err(NjallaError::RegistrationTimeout {
                domain: domain.to_string(),
                timeout_secs: timeout,
            });
        }

        let status = client.check_task(&task_id).await?;

        match status.status.as_str() {
            "completed" => {
                match format {
                    OutputFormat::Json => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "domain": domain,
                                "task_id": task_id,
                                "status": "completed"
                            }))?
                        );
                    }
                    OutputFormat::Table => {
                        println!(
                            "{} Domain {} registered successfully!",
                            "✓".green(),
                            domain.cyan()
                        );
                    }
                }
                return Ok(());
            }
            "failed" => {
                return Err(NjallaError::Api {
                    message: format!("Registration failed for {domain}"),
                });
            }
            _ => {
                // Still pending/processing, wait and retry
                sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
            }
        }
    }
}
