use chrono::{DateTime, Utc};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct MaintenanceMetrics {
    pub updated_at: Option<DateTime<Utc>>,
    pub repository_archived: Option<bool>,
    pub status: Status,
}
#[derive(Debug, Serialize)]
pub struct PopularityMetrics {
    pub downloads: u64,
    pub recent_downloads: u64,
}
#[derive(Debug, Serialize, Clone)]
pub struct SecurityMetrics {
    pub known_advisories: u32,
    pub critical_advisories: u32,
    pub high_advisories: u32,
}
#[derive(Debug, PartialEq, Serialize)]
pub enum Status {
    Active,
    Inactive,
    Unknown
}
#[derive(Ord, PartialOrd, PartialEq, Eq)]
pub enum VulnSeverity {
    Unknown,
    Low,
    Moderate,
    High,
    Critical,
}

impl From<&str> for VulnSeverity {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "CRITICAL" =>  Self::Critical,
            "HIGH" => Self::High,
            "MODERATE" => Self::Moderate,
            "LOW" => Self::Low,
            _ => Self::Unknown
        }
    }
}

// let severity: VulnSeverity = affected.ecosystem_specific.and_then(|es| es.severity).map(|s| VulnSeverity::from(&s)).unwrap_or(VulnSeverity::Unknown);