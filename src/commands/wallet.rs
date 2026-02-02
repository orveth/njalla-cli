//! Wallet management commands.

use crate::client::NjallaClient;
use crate::error::Result;
use crate::output::{format_payment, format_transactions, format_wallet_balance};
use crate::types::PaymentMethod;

/// Run the balance command.
///
/// Shows the current wallet balance.
pub fn run_balance(debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let balance = client.get_balance()?;
    let formatted = format_wallet_balance(&balance)?;
    println!("{formatted}");

    Ok(())
}

/// Run the add-payment command.
///
/// Creates a new payment to refill the wallet.
pub fn run_add_payment(amount: i32, via: PaymentMethod, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let payment = client.add_payment(amount, via)?;
    let formatted = format_payment(&payment)?;
    println!("{formatted}");

    Ok(())
}

/// Run the get-payment command.
///
/// Gets details about a specific payment.
pub fn run_get_payment(id: &str, debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let payment = client.get_payment(id)?;
    let formatted = format_payment(&payment)?;
    println!("{formatted}");

    Ok(())
}

/// Run the transactions command.
///
/// Lists transactions from the last 90 days.
pub fn run_transactions(debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;

    let transactions = client.list_transactions()?;
    let formatted = format_transactions(&transactions)?;
    println!("{formatted}");

    Ok(())
}
