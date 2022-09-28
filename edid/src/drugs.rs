use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Define {
        /// Name of a drug to define
        name: String,
    },
}

/// Run drugs subcommand
pub fn run(command: &Commands) {
    match command {
        Commands::Define { name } => {
            define_drug(name);
        }
    }
}

/// Define a new drug
fn define_drug(name: &String) {
    println!("{name} has been defined.");
}
