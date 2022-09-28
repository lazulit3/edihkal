use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Drugs {
        #[command(subcommand)]
        drugs_command: Option<DrugsCommands>,
    },
}

#[derive(Subcommand)]
enum DrugsCommands {
    Define {
        /// Name of a drug to define
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Drugs { drugs_command }) => {
            if let Some(DrugsCommands::Define { name }) = drugs_command {
                println!("{name} has been defined.");
            }
        }
        None => {}
    }
}
