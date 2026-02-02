//! Wallet management commands.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::{
    format_payment, format_transactions, format_wallet_balance, OutputFormat,
};
use crate::types::PaymentMethod;

/// Run the balance command.
///
/// Shows the current wallet balance.
pub async fn run_balance(output: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;
    let format: OutputFormat = output.parse().expect("infallible");

    let balance = client.get_balance().await?;
    let formatted = format_wallet_balance(&balance, format)?;
    println!("{formatted}");

    Ok(())
}

/// Run the add-payment command.
///
/// Creates a new payment to refill the wallet.
pub async fn run_add_payment(amount: i32, via: PaymentMethod, output: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;
    let format: OutputFormat = output.parse().expect("infallible");

    let payment = client.add_payment(amount, via).await?;
    let formatted = format_payment(&payment, format)?;
    println!("{formatted}");

    Ok(())
}

/// Run the get-payment command.
///
/// Gets details about a specific payment.
pub async fn run_get_payment(id: &str, output: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;
    let format: OutputFormat = output.parse().expect("infallible");

    let payment = client.get_payment(id).await?;
    let formatted = format_payment(&payment, format)?;
    println!("{formatted}");

    Ok(())
}

/// Run the transactions command.
///
/// Lists transactions from the last 90 days.
pub async fn run_transactions(output: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;
    let format: OutputFormat = output.parse().expect("infallible");

    let transactions = client.list_transactions().await?;
    let formatted = format_transactions(&transactions, format)?;
    println!("{formatted}");

    Ok(())
}
