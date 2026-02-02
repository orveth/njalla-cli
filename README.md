# njalla-cli

[![CI](https://github.com/orveth/njalla-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/orveth/njalla-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Privacy-first domain management CLI for [Njalla](https://njal.la), built in Rust.

> **Disclaimer:** This is an unofficial project and is not affiliated with or endorsed by Njalla. Use at your own risk. Always verify important operations through the official Njalla web interface.

## Features

| Category | Command | Description |
|----------|---------|-------------|
| **Domains** | `domains` | List all domains in your account |
| | `status <domain>` | Get domain details |
| | `status <domain> --dns` | Get domain details with DNS records |
| | `search <query>` | Search for available domains |
| | `register <domain>` | Register a new domain |
| **DNS** | `dns list <domain>` | List all DNS records |
| | `dns add <domain>` | Add a DNS record |
| | `dns edit <domain>` | Edit an existing record |
| | `dns remove <domain>` | Remove a DNS record |
| **Wallet** | `wallet balance` | Check wallet balance |
| | `wallet add-payment` | Add funds (Bitcoin) |
| | `wallet get-payment <id>` | Check payment status |
| | `wallet transactions` | List recent transactions |

**Supported DNS record types:** A, AAAA, ANAME, CAA, CNAME, DS, Dynamic, HTTPS, MX, NAPTR, NS, PTR, SRV, SSHFP, SVCB, TLSA, TXT

## Why This CLI?

**Fully synchronous** - No async runtime. Uses [bitreq](https://crates.io/crates/bitreq) for HTTP, resulting in fast compile times and a small binary (~2.3MB stripped).

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

### DNS Management

```bash
# List DNS records
njalla dns list example.com

# Add an A record
njalla dns add example.com -t a -n @ -c 1.2.3.4 --ttl 3600

# Add an MX record with priority
njalla dns add example.com -t mx -n @ -c mail.example.com --ttl 3600 -p 10

# Add an SRV record
njalla dns add example.com -t srv -n _sip._tcp -c sipserver.example.com \
  --ttl 3600 -p 10 -w 5 --port 5060

# Add a Dynamic DNS record
njalla dns add example.com -t dynamic -n home

# Edit a record
njalla dns edit example.com --id 1337 -c 5.6.7.8 --ttl 300

# Remove a record
njalla dns remove example.com --id 1337
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
