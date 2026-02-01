# njalla-cli

Privacy-first domain management CLI for [Njalla](https://njal.la), built in Rust.

> **Disclaimer:** This is an unofficial project and is not affiliated with or endorsed by Njalla. Use at your own risk. Always verify important operations through the official Njalla web interface.

## Features

- List and manage domains
- Search and register domains
- Manage DNS records
- Wallet balance and payments (BTC only)
- JSON output for scripting

## Installation

```bash
# With Nix
nix run github:orveth/njalla-cli

# From source
cargo build --release
```

## Configuration

Get your API token from https://njal.la/settings/api/

```bash
# Option 1: Config file
njalla config --init
# Then edit config.toml with your token

# Option 2: Environment variable
export NJALLA_API_TOKEN="your-token"
```

## Usage

```bash
njalla --help           # Full documentation
njalla domains          # List your domains
njalla search example   # Search available domains
njalla wallet balance   # Check wallet balance
```

## Development

This project uses Nix for reproducible builds. Install [Nix](https://determinate.systems/nix-installer/) and [direnv](https://direnv.net/), then:

```bash
direnv allow   # Auto-enters dev environment
cargo test     # Run tests
cargo build    # Build
```

## License

MIT
