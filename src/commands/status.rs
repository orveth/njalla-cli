//! Domain status command.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::{format_domain_status, OutputFormat};

/// Run the status command.
///
/// Shows detailed status for a domain.
pub async fn run(domain: &str, show_dns: bool, output: &str) -> Result<()> {
    let client = NjallaClient::new()?;
    let format = OutputFormat::from_str(output);

    let info = client.get_domain(domain).await?;
    let records = if show_dns {
        Some(client.list_records(domain).await?)
    } else {
        None
    };

    let formatted = format_domain_status(&info, records.as_deref(), format)?;
    print!("{formatted}");

    Ok(())
}
