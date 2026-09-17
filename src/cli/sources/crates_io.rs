use serde::Deserialize;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use crate::cli::sources::source::Source;
use crate::cli::models::package::Package;

#[derive(Deserialize)]
pub struct CratesIoResponse {
    #[serde(rename = "crate")]
    pub crate_data: PackageResponseData,
    pub versions: Vec<VersionData>,
}

#[derive(Deserialize)]
pub struct PackageResponseData {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub recent_downloads: u64,
    pub max_version: String,
    pub repository: Option<String>,
    pub created_at: DateTime<Utc>,
    pub keywords: Vec<String>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Deserialize)]
pub struct VersionData {
    pub num: String,
    pub updated_at: DateTime<Utc>,
    pub license: Option<String>,
    pub yanked: bool,
}

pub struct CratesIo {
    client: reqwest::Client,
}

impl CratesIo {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("trustpkg (https://github.com/Gonzaa21/trustpkg)")
            .build()
            .expect("Failed to build reqwest client");
        Self { client }
    }
}

#[async_trait]
impl Source for CratesIo {
    fn name(&self) -> &'static str  {
        "crates.io"
    }
    
    async fn fetch(&self, name: &str, version: Option<&str>) -> Result<Package, Box<dyn std::error::Error>> {
        let url: String = format!("https://crates.io/api/v1/crates/{name}");
        let response = self.client.get(&url).send().await?;
        
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(format!("Crate '{name}' not found on crates.io").into());
        }

        let data: CratesIoResponse = response.json().await?;
        let requested_version = version.unwrap_or(&data.crate_data.max_version);

        let version_data = data.versions.iter()
            .find(|v| v.num == requested_version)
            .ok_or_else(|| format!("Version '{requested_version}' not found for crate '{name}'"))?;
        
        Ok(Package {
            name: data.crate_data.name,
            version: version_data.num.clone(),
            description: data.crate_data.description,
            repository: data.crate_data.repository,
            created_at: data.crate_data.created_at,
            downloads: data.crate_data.downloads,
            recent_downloads: data.crate_data.recent_downloads,
            updated_at: data.crate_data.updated_at,
            keywords: data.crate_data.keywords,
            license: version_data.license.clone(),
        })
        
    }
}