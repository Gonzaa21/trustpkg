use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Recommendation {
    pub severity: Severity,
    pub message: String, 
    pub evidence: EvidenceKind,
}
#[derive(Debug)]
pub enum Severity {
    Info,
    Warning,
    Critical
}
#[derive(Debug)]
pub enum EvidenceKind {
    Date(DateTime<Utc>),
    Count(u32),
    Flag(bool),
    Text(String),
}