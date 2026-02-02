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
- Define `NjallaError` enum with manual Display/Error impls
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
- Format data as JSON for consistent, scriptable output
- All output uses `serde_json::to_string_pretty`

### commands/
Each command module follows the same pattern:
```rust
pub fn run(debug: bool) -> Result<()> {
    let client = NjallaClient::new(debug)?;
    let result = client.some_method()?;
    println!("{}", serde_json::to_string_pretty(&result)?);
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

## Configuration

Config file (`./config.toml`) or environment variable:
- `NJALLA_API_TOKEN` - API token (env var takes precedence)

```toml
# config.toml
api_token = "your-token-here"
```

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

    #[test]
    fn test_client_with_mock() {
        // Test with wiremock
    }
}
```

### Integration Tests
```rust
// tests/integration.rs
#[test]
#[ignore] // Requires real API token
fn test_list_domains() {
    // Real API call
}
```

## Dependencies

### Runtime
- `clap` - CLI argument parsing
- `reqwest` - HTTP client (blocking)
- `serde` / `serde_json` - JSON handling

### Development
- `tokio` / `wiremock` - HTTP mocking for tests

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

1. **Multiple accounts** - Support named profiles
2. **Shell completions** - Generate bash/zsh/fish completions
3. **DNS templates** - Apply common DNS patterns
4. **Webhook support** - Notifications on domain events
