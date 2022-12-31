use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use edihkal_client::{Client, NewDrug};

use crate::config::Config;

/// A CLI client for edihkal
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    /// Command to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage drugs known by edihkal
    #[command(alias("drugs"))]
    Drug {
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
    /// Get drugs defined in edihkal
    List,
}

/// Run appropriate command based on parsed Opts.
pub async fn run_command(opts: Opts) -> Result<(), anyhow::Error> {
    let config = Config::load()?;
    let client = Client::new(&config.edihkal_url);
    match &opts.command {
        Commands::Drug { command } => match command {
            DrugsCommands::Define { name } => {
                let drug = client
                    .define_drug(NewDrug::new(name))
                    .await
                    .context("Failed to define drug")?;
                println!("{} has been defined.", drug.name());
                Ok(())
            }
            DrugsCommands::List => {
                let drugs = client.get_drugs().await.context("Failed to get defined drugs")?;
                for drug in &drugs {
                    println!("{}", drug.name());
                }
                Ok(())
            }
        },
    }
}
