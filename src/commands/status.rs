//! Domain status command.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::format_domain_status;

/// Run the status command.
///
/// Shows detailed status for a domain.
pub fn run(domain: &str, show_dns: bool, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let info = client.get_domain(domain)?;
    let records = if show_dns {
        Some(client.list_records(domain)?)
    } else {
        None
    };

    let formatted = format_domain_status(&info, records.as_deref())?;
    println!("{formatted}");

    Ok(())
}
