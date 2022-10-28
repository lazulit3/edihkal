use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use edihkal_client::Client;
use edihkal_core::drugs::Drug;

use crate::config::Config;

/// A CLI client for edihkal
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    /// Use a config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

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

impl Opts {
    /// Returns Path of config file parsed from CLI options (if specified).
    pub fn config_path(&self) -> Option<&Path> {
        self.config.as_deref()
    }
}

/// Run appropriate command based on parsed Opts.
pub async fn run_command(opts: Opts) -> Result<(), anyhow::Error> {
    match &opts.command {
        Commands::Drugs { command } => match command {
            DrugsCommands::Define { name } => {
                let config = Config::from(&opts);
                let client = Client::new(&config.edihkal_url);
                let response = client.create_drug(name).context("Failed to define drug")?;
                let drug: Drug = response.data;
                println!("{} has been defined.", drug.name);
                Ok(())
            }
        },
    }
}
