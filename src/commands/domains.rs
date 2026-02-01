//! List domains command.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::{format_domains, OutputFormat};

/// Run the domains command.
///
/// Lists all domains in the user's Njalla account.
pub async fn run(output: &str) -> Result<()> {
    let client = NjallaClient::new()?;
    let format = OutputFormat::from_str(output);

    let domains = client.list_domains().await?;
    let formatted = format_domains(&domains, format)?;
    print!("{formatted}");

    Ok(())
}
