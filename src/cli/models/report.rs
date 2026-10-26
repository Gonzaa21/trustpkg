use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Recommendation {
    pub severity: Severity,
    pub message: String, 
    pub evidence: EvidenceKind,
}
#[derive(Debug, Serialize)]
pub enum Severity {
    Info,
    Warning,
    Critical
}
#[derive(Debug, Serialize)]
pub enum EvidenceKind {
    Date(DateTime<Utc>),
    Count(u32),
    Flag(bool),
    Text(String),
}