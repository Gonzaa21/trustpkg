use crate::cli::models::package::Package;
use crate::cli::models::metrics::{SecurityMetrics, MaintenanceMetrics, PopularityMetrics};
use crate::cli::models::score::TrustScore;
use crate::cli::models::report::Recommendation;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Metadata { 
    pub analyzed_at: DateTime<Utc>, 
    pub sources: String, 
    pub trustpkg_version: String,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub package: Package,
    pub metadata: Metadata,
    pub security: SecurityMetrics,
    pub maintenance: MaintenanceMetrics,
    pub popularity: PopularityMetrics,
    pub score: TrustScore,
    pub recommendations: Vec<Recommendation>
}