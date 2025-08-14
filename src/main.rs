//! Garnix Fetcher CLI Binary
//!
//! A command-line tool for fetching build status information from Garnix.io.

use garnix_fetcher::cli::Cli;
use clap::Parser;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = cli.run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
