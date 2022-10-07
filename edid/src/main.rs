mod cli;
mod client;
mod configuration;

use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let opts = cli::Opts::parse();
    cli::run_command(opts).await;
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    cli::Opts::command().debug_assert()
}
