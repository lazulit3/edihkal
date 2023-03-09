mod cli;
mod client;
mod cmd;
mod config;

use client::client;

use anyhow::Result;
use clap::Parser;
use edihkal_tracing::configure_tracing;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    configure_tracing("edid", "off", std::io::stdout);
    let opts = cli::Opts::parse();
    cli::run_command(opts).await
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    cli::Opts::command().debug_assert()
}
