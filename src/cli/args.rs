use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::{Display, Formatter, Result};

// CLI struct
#[derive(Parser)]
#[command(name = "trustpkg")]
#[command(about = "Trust Dependency Analyzer", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

// Commands
#[derive(Subcommand)]
pub enum Commands {
    Analyze {
        name: String,
        #[arg(long, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long)]
        version: Option<String>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Text,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Text => write!(f, "text"),
        }
    }
}