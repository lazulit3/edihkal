pub mod new_entry;

use self::new_entry::new_entry;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Entry {
    /// Create a new journal entry (record of drug use)
    New {
        /// Quantity of the drug that was used
        dose: i32,
        /// Name of the drug that was used
        drug_name: String,
        /// When the drug was used
        #[arg(trailing_var_arg = true)]
        when: Vec<String>,
    },
}

pub async fn execute(command: &Entry) -> Result<(), anyhow::Error> {
    match command {
        Entry::New {
            when,
            drug_name: drug,
            dose,
        } => {
            let when = when.join(" ");
            new_entry(*dose, drug, &when).await
        }
    }
}
