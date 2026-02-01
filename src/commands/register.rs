//! Register domain command.

use crate::error::{NjallaError, Result};

/// Run the register command.
///
/// Registers a new domain through Njalla.
#[allow(clippy::too_many_arguments, clippy::unused_async)]
pub async fn run(
    _domain: &str,
    _years: i32,
    _confirm: bool,
    _wait: bool,
    _timeout: u64,
    _output: &str,
    _debug: bool,
) -> Result<()> {
    // TODO: Implement in Phase 4
    Err(NjallaError::NotImplemented("register".to_string()))
}
