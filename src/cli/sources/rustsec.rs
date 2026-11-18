use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::cli::cache::Cache;

#[derive(Serialize)]
pub struct OsvQuery { 
    package: OsvPackageResponseData, 
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>
}

#[derive(Deserialize)]
pub struct RustSecResponse {
    #[serde(default)]
    pub vulns: Vec<VulnsData>,
}

#[derive(Serialize, Deserialize)]
pub struct OsvPackageResponseData {
    name: String,
    ecosystem: String,
}
#[derive(Deserialize)]
pub struct VulnsData {
    pub id: String,
    #[serde(default)]
    pub affected: Vec<AffectedData>,
}

#[derive(Deserialize)]
pub struct AffectedData {
    #[serde(default)]
    pub ecosystem_specific: Option<EcosystemSpecific>,
}

#[derive(Deserialize)]
pub struct EcosystemSpecific {
    pub severity: Option<String>,
}

pub struct Rustsec {
    client: reqwest::Client,
}

impl Rustsec {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("trustpkg (https://github.com/Gonzaa21/trustpkg)")
            .build()
            .expect("Failed to build reqwest client");
        Self { client }
    }

    pub async fn fetch_advisories(&self, cache: &Cache, name: String, version: String) -> Result<RustSecResponse, Box<dyn std::error::Error>> {
        const RUSTSEC_TTL: Duration = Duration::from_secs(60 * 60);
    
        let key = format!("osv/{name}/{version}");
        let cached = cache
            .get(&key, RUSTSEC_TTL)
            .and_then(|text| serde_json::from_str::<RustSecResponse>(&text).ok());
    
        if let Some(data) = cached {
            return Ok(data);
        }
    
        let body = OsvQuery {
            package: OsvPackageResponseData { name, ecosystem: "crates.io".to_string() },
            version,
            commit: None,
            page_token: None,
        };
    
        let response = self.client.post("https://api.osv.dev/v1/query").json(&body).send().await?;
        if !response.status().is_success() {
            return Err(format!("OSV API request failed with status: {}", response.status()).into());
        }
    
        let text = response.text().await?;
        let data: RustSecResponse = serde_json::from_str(&text)?;
        cache.put(&key, &text);
        Ok(data)
    }
}