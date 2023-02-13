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
    /// Manage drugs known by edihkal
    #[command(alias("drugs"))]
    Drug {
        #[command(subcommand)]
        cmd: cmd::Drug,
    },
}

/// Run appropriate command based on parsed Opts.
pub async fn run_command(opts: Opts) -> Result<(), anyhow::Error> {
    match &opts.cmd {
        Commands::Drug { cmd } => cmd::drug::execute(cmd).await,
    }
}
