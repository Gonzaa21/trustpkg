use clap::Parser;
use crate::cli::args::{Cli, Commands};
use crate::cli::analysis::engine::run_analysis;
use crate::cli::models::analysis::AnalysisResult;
use crate::cli::args::OutputFormat;
use crate::cli::report::terminal::render;
use crate::cli::report::json;

pub async fn run() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Analyze { name, version, format } => {
            match run_analysis(name, version).await {
                Ok(result) => handle_output(&result, format),
                Err(e) => println!("{e}")
            }
        }
    }
}

pub fn handle_output(result: &AnalysisResult, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            let rendered = render(result);
            print!("{}", rendered);
        }
        OutputFormat::Json => {
            match json::render(result) {
                Ok(json_str) => println!("{}", json_str),
                Err(e) => eprintln!("Error serializing the result to JSON: {}", e),
            }
        }
    }
}