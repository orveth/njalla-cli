//! Validate domain registration command.

use crate::error::{NjallaError, Result};

/// Run the validate command.
///
/// Validates that a domain was properly registered.
#[allow(clippy::unused_async)]
pub async fn run(_domain: &str, _output: &str) -> Result<()> {
    // TODO: Implement in Phase 5
    Err(NjallaError::NotImplemented("validate".to_string()))
}
