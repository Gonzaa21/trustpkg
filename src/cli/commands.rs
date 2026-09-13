use clap::Parser;
use crate::cli::args::{Cli, Commands};
use crate::cli::analysis::engine::run_analysis;

pub async fn run() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Analyze { name, version, format } => {
            match run_analysis(name, version).await {
                Ok(result) => println!("{:?}", result),
                Err(e) => println!("{e}")
            }
        }
    }
}