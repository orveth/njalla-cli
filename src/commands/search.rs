//! Search domains command.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::{format_market_domains, OutputFormat};

/// Run the search command.
///
/// Searches for available domains matching the query.
pub async fn run(query: &str, output: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;
    let format: OutputFormat = output.parse().expect("infallible");

    let results = client.find_domains(query).await?;
    let formatted = format_market_domains(&results, format)?;
    println!("{formatted}");

    Ok(())
}
