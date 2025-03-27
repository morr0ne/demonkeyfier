use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Extract,
}

fn main() {
    let Cli { commands } = Cli::parse();

    match commands {
        Commands::Extract => {
            println!("Extracting...")
        }
    }
}
