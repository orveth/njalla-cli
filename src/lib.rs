//! njalla-cli library.
//!
//! This crate provides a Rust client for the Njalla domain management API.
//!
//! # Example
//!
//! ```no_run
//! use njalla_cli::client::NjallaClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = NjallaClient::new()?;
//!     let domains = client.list_domains().await?;
//!     println!("Found {} domains", domains.len());
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod output;
pub mod types;
