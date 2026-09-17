use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Package { 
    pub name: String, 
    pub version: String, 
    pub description: Option<String>,
    pub repository: Option<String>,
    pub created_at: DateTime<Utc>,
    pub downloads: u64,
    pub recent_downloads: u64,
    pub updated_at: DateTime<Utc>,
    pub keywords: Vec<String>,
    pub license: Option<String>,
}