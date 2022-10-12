use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::client::EdihkalClient;
use crate::config::Config;

/// A CLI client for edihkal
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    /// Use a config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Command to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage drugs known by edihkal
    Drugs {
        #[command(subcommand)]
        command: DrugsCommands,
    },
}

#[derive(Subcommand)]
enum DrugsCommands {
    /// Define a new drug in edihkal
    Define {
        /// Name of the drug
        name: String,
    },
}

impl From<&Opts> for Config {
    fn from(opts: &Opts) -> Self {
        if let Some(config_path) = opts.config.as_deref() {
            Config::load_with_config_file(config_path)
        } else {
            Config::load()
        }
        .expect("Failed to load configuration")
    }
}

/// Run appropriate command based on Opts
pub async fn run_command(opts: Opts) {
    match &opts.command {
        Commands::Drugs { command } => match command {
            DrugsCommands::Define { name } => {
                let config = Config::from(&opts);
                let client = EdihkalClient::from(&config);
                match client.define_drug(name).await {
                    Ok(_) => println!("Defined drug {}.", name),
                    // TODO: Exit with error status.
                    // TODO: Clear error handling / output.
                    Err(_) => println!("Failed to define drug {}!", name),
                }
            }
        },
    }
}
