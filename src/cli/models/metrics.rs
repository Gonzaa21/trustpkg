use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MaintenanceMetrics {
    pub last_release: DateTime<Utc>,
    pub repository_archived: bool,
    pub status: Status,
}
#[derive(Debug)]
pub struct PopularityMetrics {
    pub downloads: i32,
    pub dependent_packages: i32,
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
    Inactive
}