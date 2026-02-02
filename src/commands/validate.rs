//! Validate domain registration command.

use crate::error::{NjallaError, Result};

/// Run the validate command.
///
/// Validates that a domain was properly registered.
pub fn run(_domain: &str, _debug: bool) -> Result<()> {
    // TODO: Implement in Phase 5
    Err(NjallaError::NotImplemented("validate".to_string()))
}
