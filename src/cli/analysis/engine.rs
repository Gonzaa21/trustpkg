use crate::cli::models::analysis::AnalysisResult;
use crate::cli::models::report::{EvidenceKind, Recommendation, Severity};
use crate::cli::sources::{crates_io::CratesIo, source::Source};
use crate::cli::score::{popularity::calculate_popularity, maintenance::calculate_maintenance, security::calculate_security, calculator::{calculate_score, score_security}};
use crate::cli::models::analysis::Metadata;
use chrono::Utc;

pub async fn run_analysis(
    name: String,
    version: Option<String>,
) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
    let source = CratesIo::new();
    let package = source.fetch(&name, version.as_deref()).await?;
    
    let pop = calculate_popularity(&package);
    let maint = calculate_maintenance(&package).await;
    
    let mut recommendations = Vec::new();
    let mut sources = String::from("crates.io");
    
    let sec = match calculate_security(&package).await {
        Ok(m) => {
            sources.push_str(", osv.dev");
            Some(m)
        }
        Err(e) => {
            recommendations.push(Recommendation {
                severity: Severity::Warning,
                message: "Security could not be verified; score is partial".to_string(),
                evidence: EvidenceKind::Text(e.to_string()),
            });
            None
        }
    };
    let score = calculate_score(sec.as_ref().map(score_security), &maint, &pop);

    let metadata = Metadata {
        analyzed_at: Utc::now(),
        sources,
        trustpkg_version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(AnalysisResult {
        package,
        metadata,
        security: sec,
        maintenance: maint,
        popularity: pop,
        score,
        recommendations,
    })
}