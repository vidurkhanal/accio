mod cli;
mod client;
mod command;

use clap::Parser;
use cli::CLI;
use client::Client;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = CLI::parse();
    let accio_client = Client::new("[::1]:55001".to_string());
    accio_client.run(args).await?;
    Ok(())
}
