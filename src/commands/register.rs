//! Register domain command.

use crate::client::NjallaClient;
use crate::error::{NjallaError, Result};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

/// Poll interval for checking task status.
const POLL_INTERVAL_SECS: u64 = 2;

/// Run the register command.
///
/// Registers a new domain through Njalla.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn run(
    domain: &str,
    years: i32,
    confirm: bool,
    wait: bool,
    timeout: u64,
    debug: bool,
) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    // Check domain availability and get price
    let search_results = client.find_domains(domain)?;
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
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "domain": domain,
                "price_per_year": info.price,
                "years": years,
                "total_price": total_price
            }))?
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
    let task_id = client.register_domain(domain, years)?;

    if !wait {
        // Output task ID and exit
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "domain": domain,
                "task_id": task_id,
                "status": "pending"
            }))?
        );
        return Ok(());
    }

    // Poll for completion
    eprintln!("Waiting for registration to complete...");
    let start = Instant::now();
    let timeout_duration = Duration::from_secs(timeout);

    loop {
        if start.elapsed() > timeout_duration {
            return Err(NjallaError::RegistrationTimeout {
                domain: domain.to_string(),
                timeout_secs: timeout,
            });
        }

        let status = client.check_task(&task_id)?;

        match status.status.as_str() {
            "completed" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "domain": domain,
                        "task_id": task_id,
                        "status": "completed"
                    }))?
                );
                return Ok(());
            }
            "failed" => {
                return Err(NjallaError::Api {
                    message: format!("Registration failed for {domain}"),
                });
            }
            _ => {
                // Still pending/processing, wait and retry
                thread::sleep(Duration::from_secs(POLL_INTERVAL_SECS));
            }
        }
    }
}
