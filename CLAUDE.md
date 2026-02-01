# Development Guide

This project uses Nix with direnv for development environment management.

## Building and Testing

With direnv enabled, cargo commands work directly:

```bash
cargo build        # Build
cargo test         # Run tests
cargo clippy       # Lint
cargo run -- <args> # Run the CLI
nix build          # Build release
```

## Examples

```bash
cargo run wallet balance              # Check wallet balance
cargo run -- --debug wallet transactions  # List transactions with debug output
cargo run -- --help                   # Get help
```

## Configuration

The CLI requires a Njalla API token. Configure it via:

1. Environment variable: `export NJALLA_API_TOKEN="your-token"`
2. Config file: `./config.toml` with `api_token = "your-token"`
