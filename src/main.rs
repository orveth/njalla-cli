//! njalla-cli - Privacy-first domain management CLI for Njalla.

mod client;
mod commands;
mod config;
mod error;
mod output;
mod types;

use clap::{Parser, Subcommand};

/// Privacy-first domain management CLI for Njalla.
#[derive(Parser)]
#[command(name = "njalla")]
#[command(author, version, about)]
#[command(long_about = "Privacy-first domain management CLI for Njalla.\n\n\
Manage your domains, DNS records, and wallet from the command line.")]
#[command(after_help = "\
CONFIGURATION:
    Get your API token from https://njal.la/settings/api/

    Option 1: Config file (recommended)
        njalla config --init    # Creates ./config.toml
        Edit the file to add your token

    Option 2: Environment variable
        export NJALLA_API_TOKEN=\"your-token\"

    Environment variable takes precedence over config file.

EXAMPLES:
    njalla domains                      List all your domains
    njalla search bitcoin               Search for available domains
    njalla register example.com         Register a domain (interactive)
    njalla register example.com --wait  Register and wait for completion
    njalla status example.com --dns     Show domain status with DNS records
    njalla wallet balance               Check wallet balance
    njalla wallet add-payment -a 15 -v btc   Add funds via Bitcoin

MORE INFO:
    https://github.com/gudnuf/njalla-cli
    https://njal.la/api/")]
struct Cli {
    /// Enable debug mode to see raw API responses.
    #[arg(long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all domains in your account.
    Domains,

    /// Search for available domains.
    Search {
        /// Domain name or keyword to search.
        query: String,
    },

    /// Register a new domain.
    ///
    /// Requires sufficient balance in your Njalla wallet.
    /// Top up at <https://njal.la/wallet/>
    Register {
        /// Domain name to register (e.g., example.com).
        domain: String,

        /// Registration period in years (1-10).
        #[arg(short, long, default_value = "1", value_parser = clap::value_parser!(i32).range(1..=10))]
        years: i32,

        /// Skip confirmation prompt.
        #[arg(long)]
        confirm: bool,

        /// Wait for registration to complete.
        #[arg(long)]
        wait: bool,

        /// Timeout for --wait in seconds.
        #[arg(long, default_value = "300")]
        timeout: u64,
    },

    /// Check domain status and details.
    Status {
        /// Domain name to check.
        domain: String,

        /// Include DNS records in output.
        #[arg(long)]
        dns: bool,
    },

    /// Validate that a domain was properly registered.
    ///
    /// Checks: exists in account, status is active, has expiry, DNS accessible.
    Validate {
        /// Domain name to validate.
        domain: String,
    },

    /// Show or initialize configuration.
    Config {
        /// Initialize config file if it doesn't exist.
        #[arg(long)]
        init: bool,
    },

    /// Manage wallet and payments.
    Wallet {
        #[command(subcommand)]
        command: WalletCommands,
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Show current wallet balance.
    Balance,

    /// Add payment to refill wallet.
    AddPayment {
        /// Amount in EUR (5 or multiple of 15, max 300).
        #[arg(short, long)]
        amount: i32,

        /// Payment method.
        #[arg(short, long, value_enum)]
        via: types::PaymentMethod,
    },

    /// Get details about a payment.
    GetPayment {
        /// Payment ID.
        id: String,
    },

    /// List transactions from the last 90 days.
    Transactions,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Domains => commands::domains::run(cli.debug),
        Commands::Search { query } => commands::search::run(&query, cli.debug),
        Commands::Register {
            domain,
            years,
            confirm,
            wait,
            timeout,
        } => commands::register::run(&domain, years, confirm, wait, timeout, cli.debug),
        Commands::Status { domain, dns } => commands::status::run(&domain, dns, cli.debug),
        Commands::Validate { domain } => commands::validate::run(&domain, cli.debug),
        Commands::Config { init } => run_config(init),
        Commands::Wallet { command } => match command {
            WalletCommands::Balance => commands::wallet::run_balance(cli.debug),
            WalletCommands::AddPayment { amount, via } => {
                commands::wallet::run_add_payment(amount, via, cli.debug)
            }
            WalletCommands::GetPayment { id } => commands::wallet::run_get_payment(&id, cli.debug),
            WalletCommands::Transactions => commands::wallet::run_transactions(cli.debug),
        },
    }
}

fn run_config(init: bool) -> error::Result<()> {
    use std::path::Path;

    let config_path = Path::new("config.toml");

    if init {
        if config_path.exists() {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "status": "exists",
                    "path": "./config.toml",
                    "message": "Config file already exists"
                }))?
            );
            return Ok(());
        }

        let template = r#"# Njalla CLI Configuration
# Get your API token from: https://njal.la → Settings → API

api_token = ""
"#;
        std::fs::write(config_path, template).map_err(|e| error::NjallaError::Config {
            message: format!("Failed to write config file: {e}"),
        })?;

        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "status": "created",
                "path": "./config.toml",
                "message": "Config file created. Edit to add your API token from https://njal.la/settings/api/"
            }))?
        );
        return Ok(());
    }

    // Show current config status
    let config = config::Config::load()?;

    let token_info = if let Ok(token) = config.api_token() {
        // Show masked token
        let masked = if token.len() > 8 {
            format!("{}...{}", &token[..4], &token[token.len() - 4..])
        } else {
            "****".to_string()
        };
        serde_json::json!({
            "configured": true,
            "masked_token": masked,
            "source": if std::env::var("NJALLA_API_TOKEN").is_ok() { "env" } else { "config file" }
        })
    } else {
        serde_json::json!({
            "configured": false,
            "message": "Run 'njalla config --init' to create a config file"
        })
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "config_file": "./config.toml",
            "file_exists": config_path.exists(),
            "api_token": token_info
        }))?
    );

    Ok(())
}
