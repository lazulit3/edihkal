use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cmd;

/// A CLI client for edihkal
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Drugs defined in edihkal
    #[command(alias("drugs"))]
    Drug {
        #[command(subcommand)]
        cmd: cmd::Drug,
    },
    /// Drug journal entries recording when quantities of drugs are used
    #[command(alias("entries"))]
    Entry {
        #[command(subcommand)]
        command: cmd::Entry,
    },
}

/// Run appropriate command based on parsed Opts.
pub async fn run_command(opts: Opts) -> Result<(), anyhow::Error> {
    match &opts.cmd {
        Commands::Drug { cmd } => cmd::drug::execute(cmd).await,
        Commands::Entry { command } => cmd::entry::execute(command).await,
    }
}
