use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MaintenanceMetrics {
    pub updated_at: Option<DateTime<Utc>>,
    pub repository_archived: Option<bool>,
    pub status: Status,
}
#[derive(Debug)]
pub struct PopularityMetrics {
    pub downloads: u64,
    pub recent_downloads: u64,
}
#[derive(Debug)]
pub struct SecurityMetrics {
    pub known_advisories: u32,
    pub critical_advisories: u32,
    pub high_advisories: u32,
}
#[derive(Debug)]
pub enum Status {
    Active,
    Inactive,
    Unknown
}