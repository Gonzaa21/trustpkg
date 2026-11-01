use clap::Parser;
use crate::cli::args::{Cli, Commands};
use crate::cli::analysis::engine::run_analysis;
use crate::cli::models::analysis::AnalysisResult;
use crate::cli::args::OutputFormat;
use crate::cli::report::{terminal::render, json};

pub async fn run() {
    let cli = Cli::parse();

    let result = match cli.commands {
        Commands::Analyze { name, version, format } => {
            match run_analysis(name, version).await {
                Ok(analysis_result) => handle_output(&analysis_result, format),
                Err(e) => Err(e),
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

pub fn handle_output(result: &AnalysisResult, format: OutputFormat) 
-> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Text => {
            let rendered = render(result);
            print!("{}", rendered);
            Ok(())
        }
        OutputFormat::Json => {
            let json_str = json::render(result)?;
            println!("{}", json_str);
            Ok(())
        }
    }
}