# njalla-cli

Privacy-first domain management CLI for [Njalla](https://njal.la), built in Rust with Nix packaging.

## Features

- List and manage domains
- Search for available domains
- Register domains (uses Njalla wallet balance)
- Manage DNS records
- Full JSON output for scripting/automation

## Installation

### With Nix (recommended)

```bash
# Run directly
nix run github:gudnuf/njalla-cli

# Install to profile
nix profile install github:gudnuf/njalla-cli

# Development shell
nix develop
```

### From source

```bash
cargo build --release
```

## Configuration

Set your Njalla API token (get it from https://njal.la → Settings → API):

```bash
export NJALLA_API_TOKEN="your-token-here"
```

## Usage

```bash
# List all domains
njalla domains

# Search for available domains
njalla search bitcoin-oracle

# Register a domain
njalla register example.com --years 1

# Check domain status
njalla status example.com --dns

# Validate registration
njalla validate example.com

# JSON output for scripting
njalla domains -o json
```

## Development

See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for build phases and contributing guidelines.

## License

MIT
