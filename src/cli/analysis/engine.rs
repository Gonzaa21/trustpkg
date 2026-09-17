use crate::cli::models::analysis::AnalysisResult;
use crate::cli::sources::crates_io::CratesIo;
use crate::cli::sources::source::Source;

pub async fn run_analysis(
    name: String,
    version: Option<String>,
) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
    //let source = CratesIo::new();
    //let package = source.fetch(&name, version.as_deref()).await?;
    //Ok(package.into())
    todo!()
}