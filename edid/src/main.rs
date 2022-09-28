use clap::{Parser, Subcommand};

mod drugs;
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
        drugs_command: drugs::Commands,
    },
}

fn main() {
    let cli = Cli::parse();

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
