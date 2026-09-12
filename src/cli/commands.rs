use crate::cli::args::{Cli, Commands};
use clap::Parser;

pub async fn run() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Analyze { name, version, format } => {
            todo!();
        }
    }
}