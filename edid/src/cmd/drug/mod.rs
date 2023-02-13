mod define_drug;
mod list_drugs;

use define_drug::define_drug;
use list_drugs::list_drugs;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Drug {
    /// Define a new drug in edihkal
    #[command(alias("new"))]
    Define {
        /// Name of the drug
        name: String,
    },
    /// Get drugs defined in edihkal
    List,
}

pub async fn execute(command: &Drug) -> Result<(), anyhow::Error> {
    match command {
        Drug::Define { name } => define_drug(name).await,
        Drug::List => list_drugs().await,
    }
}
