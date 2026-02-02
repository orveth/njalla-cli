//! DNS record management commands.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::{format_record, format_records};
use crate::types::{AddRecordParams, EditRecordParams};

/// Run the dns list command.
///
/// Lists all DNS records for a domain.
pub fn run_list(domain: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let records = client.list_records(domain)?;
    let formatted = format_records(&records)?;
    println!("{formatted}");

    Ok(())
}

/// Run the dns add command.
///
/// Adds a new DNS record to a domain.
pub fn run_add(params: &AddRecordParams, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let record = client.add_record(params)?;
    let formatted = format_record(&record)?;
    println!("{formatted}");

    Ok(())
}

/// Run the dns edit command.
///
/// Edits an existing DNS record.
pub fn run_edit(params: &EditRecordParams, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let record = client.edit_record(params)?;
    let formatted = format_record(&record)?;
    println!("{formatted}");

    Ok(())
}

/// Run the dns remove command.
///
/// Removes a DNS record from a domain.
pub fn run_remove(domain: &str, id: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    client.remove_record(domain, id)?;
    println!("{}", serde_json::json!({"status": "removed", "id": id}));

    Ok(())
}
