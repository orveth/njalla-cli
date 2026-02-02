//! List domains command.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::format_domains;

/// Run the domains command.
///
/// Lists all domains in the user's Njalla account.
pub fn run(debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let domains = client.list_domains()?;
    let formatted = format_domains(&domains)?;
    println!("{formatted}");

    Ok(())
}
