mod cli;
mod client;
mod configuration;

use clap::Parser;

fn main() {
    let opts = cli::Opts::parse();
    cli::run_command(opts);
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    cli::Opts::command().debug_assert()
}
