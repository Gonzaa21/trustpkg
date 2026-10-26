use crate::cli::models::analysis::AnalysisResult;

pub fn render(result: &AnalysisResult) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(result)
}