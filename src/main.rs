//! njalla-cli - Privacy-first domain management CLI for Njalla.

mod client;
mod commands;
mod config;
mod error;
mod output;
mod types;

use clap::{Parser, Subcommand};
use colored::Colorize;

/// Privacy-first domain management CLI for Njalla.
#[derive(Parser)]
#[command(name = "njalla")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output format: table or json.
    #[arg(short, long, default_value = "table", global = true)]
    output: String,

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
    /// Top up at https://njal.la/wallet/
    Register {
        /// Domain name to register (e.g., example.com).
        domain: String,

        /// Registration period in years (1-10).
        #[arg(short, long, default_value = "1")]
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
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), err);
        std::process::exit(1);
    }
}

async fn run() -> error::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Domains => {
            commands::domains::run(&cli.output).await
        }
        Commands::Search { query } => {
            commands::search::run(&query, &cli.output).await
        }
        Commands::Register {
            domain,
            years,
            confirm,
            wait,
            timeout,
        } => {
            commands::register::run(&domain, years, confirm, wait, timeout, &cli.output).await
        }
        Commands::Status { domain, dns } => {
            commands::status::run(&domain, dns, &cli.output).await
        }
        Commands::Validate { domain } => {
            commands::validate::run(&domain, &cli.output).await
        }
        Commands::Config { init } => {
            run_config(init)
        }
    }
}

fn run_config(init: bool) -> error::Result<()> {
    use colored::Colorize;

    if init {
        let path = config::Config::init()?;
        println!("{} Config file created at:", "✓".green());
        println!("  {}", path.display());
        println!();
        println!("Edit this file to add your API token:");
        println!("  api_token = \"your-token-here\"");
        println!();
        println!("Get your token from: https://njal.la → Settings → API");
        return Ok(());
    }

    // Show current config status
    let path = config::Config::config_path()?;
    let config = config::Config::load()?;

    println!("{}", "Configuration".bold());
    println!();
    println!("{}: {}", "Config file".bold(), path.display());
    println!("{}: {}", "File exists".bold(), path.exists());
    println!();

    match config.api_token() {
        Ok(token) => {
            // Show masked token
            let masked = if token.len() > 8 {
                format!("{}...{}", &token[..4], &token[token.len()-4..])
            } else {
                "****".to_string()
            };
            println!("{}: {} (from {})",
                "API token".bold(),
                masked.green(),
                if std::env::var("NJALLA_API_TOKEN").is_ok() { "env" } else { "config file" }
            );
        }
        Err(_) => {
            println!("{}: {}", "API token".bold(), "not configured".red());
            println!();
            println!("Run {} to create a config file", "njalla config --init".cyan());
        }
    }

    Ok(())
}
