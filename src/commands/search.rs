//! Search domains command.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::format_market_domains;

/// Run the search command.
///
/// Searches for available domains matching the query.
pub fn run(query: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let results = client.find_domains(query)?;
    let formatted = format_market_domains(&results)?;
    println!("{formatted}");

    Ok(())
}
