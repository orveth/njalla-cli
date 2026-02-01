# Architecture

High-level design of njalla-cli.

## Overview

```
┌─────────────────────────────────────────────────────────┐
│                        CLI (clap)                        │
│  main.rs - Argument parsing, command dispatch           │
└─────────────────────────┬───────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────┐
│                      Commands                            │
│  commands/*.rs - Business logic per command             │
└─────────────────────────┬───────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────┐
│                      Client                              │
│  client.rs - API communication, request/response        │
└─────────────────────────┬───────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────┐
│                    Njalla API                            │
│  https://njal.la/api/1/                                 │
└─────────────────────────────────────────────────────────┘
```

## Components

### main.rs
- Parse CLI arguments with clap
- Dispatch to appropriate command handler
- Handle top-level errors and exit codes

### lib.rs
- Export public API for library use
- Re-export client, types, error modules

### error.rs
- Define `NjallaError` enum with thiserror
- Cover: auth, network, API, validation errors
- Implement Display for user-friendly messages

### types.rs
- Define all API request/response types
- Use serde for JSON serialization
- Document each type's purpose and API mapping

### client.rs
- `NjallaClient` struct with reqwest client
- `new()` - Initialize from environment
- `request()` - Generic JSON-RPC caller
- Domain methods: list, get, find, register
- Task methods: check_task
- Record methods: list, add, edit, remove

### output.rs
- Format data for table or JSON output
- Color coding for terminal (with `colored`)
- Respect `--output` flag

### commands/
Each command module follows the same pattern:
```rust
pub async fn run(args: &Args, output: &str) -> Result<()> {
    let client = NjallaClient::new()?;
    let result = client.some_method().await?;
    format_output(result, output)?;
    Ok(())
}
```

## Error Handling

```rust
// Errors propagate with ?
// Commands return Result<(), NjallaError>
// main.rs converts to exit codes

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

## Async Runtime

- Use tokio with `#[tokio::main]`
- All API calls are async
- Commands await client methods

## Configuration

Environment variables only (no config file):
- `NJALLA_API_TOKEN` - Required API token
- `NJALLA_TIMEOUT` - Optional request timeout (future)

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        // Test error formatting
    }
    
    #[tokio::test]
    async fn test_client_with_mock() {
        // Test with wiremock
    }
}
```

### Integration Tests
```rust
// tests/integration.rs
#[tokio::test]
#[ignore] // Requires real API token
async fn test_list_domains() {
    // Real API call
}
```

## Dependencies

### Runtime
- `clap` - CLI argument parsing
- `reqwest` - HTTP client
- `serde` / `serde_json` - JSON handling
- `tokio` - Async runtime
- `thiserror` - Error derivation
- `chrono` - DateTime handling
- `colored` - Terminal colors

### Development
- `wiremock` - HTTP mocking for tests

## Nix Build

```nix
rustPlatform.buildRustPackage {
  pname = "njalla-cli";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
}
```

## Future Considerations

1. **Config file** - TOML config as alternative to env vars
2. **Multiple accounts** - Support named profiles
3. **Completion** - Generate shell completions
4. **DNS templates** - Apply common DNS patterns
5. **Webhook support** - Notifications on domain events
