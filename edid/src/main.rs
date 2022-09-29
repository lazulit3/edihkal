use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod configuration;
mod drugs;
use configuration::Config;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use a config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Drugs {
        #[command(subcommand)]
        drugs_command: drugs::Commands,
    },
}

fn main() {
    let cli = Cli::parse();

    let config = if let Some(config_path) = cli.config.as_deref() {
        Config::load_with_config_file(&config_path)
    } else {
        Config::load()
    }.expect("Failed to load configuration");

    match &cli.command {
        Some(Commands::Drugs { drugs_command }) => {
            drugs::run(drugs_command);
        }
        None => {}
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
