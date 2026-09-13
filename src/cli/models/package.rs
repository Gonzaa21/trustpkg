use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Package { 
    pub name: String, 
    pub version: String, 
    pub description: String,
    pub repository: String,
    pub created_at: DateTime<Utc>,
    pub keywords: Vec<String>,
    pub license: String,
}