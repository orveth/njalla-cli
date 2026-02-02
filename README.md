# njalla-cli

[![CI](https://github.com/orveth/njalla-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/orveth/njalla-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Privacy-first domain management CLI for [Njalla](https://njal.la), built in Rust.

> **Disclaimer:** This is an unofficial project and is not affiliated with or endorsed by Njalla. Use at your own risk. Always verify important operations through the official Njalla web interface. See the [Njalla API documentation](https://njal.la/api/) for details on the underlying API.

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

### NixOS Module

```nix
{
  inputs.njalla.url = "github:orveth/njalla-cli";

  outputs = { nixpkgs, njalla, ... }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      modules = [
        njalla.nixosModules.default
        {
          programs.njalla = {
            enable = true;
            package = njalla.packages.x86_64-linux.default;
            secretsFile = "/run/secrets/njalla"; # optional, contains NJALLA_API_TOKEN=...
          };
        }
      ];
    };
  };
}
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

### Full CLI Reference

```
$ njalla --help
Usage: njalla [OPTIONS] <COMMAND>

Commands:
  domains   List all domains in your account
  search    Search for available domains
  register  Register a new domain
  status    Check domain status and details
  config    Show or initialize configuration
  dns       Manage DNS records for a domain
  wallet    Manage wallet and payments
  help      Print this message or the help of the given subcommand(s)

Options:
      --debug    Enable debug mode to see raw API responses
  -h, --help     Print help (see a summary with '-h')
  -V, --version  Print version

CONFIGURATION:
    Get your API token from https://njal.la/settings/api/

    Option 1: Config file (recommended)
        njalla config --init    # Creates ./config.toml
        Edit the file to add your token

    Option 2: Environment variable
        export NJALLA_API_TOKEN="your-token"

    Environment variable takes precedence over config file.

EXAMPLES:
    njalla domains                      List all your domains
    njalla search bitcoin               Search for available domains
    njalla register example.com         Register a domain (interactive)
    njalla register example.com --wait  Register and wait for completion
    njalla status example.com --dns     Show domain status with DNS records
    njalla wallet balance               Check wallet balance
    njalla wallet add-payment -a 15 -v btc   Add funds via Bitcoin
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
