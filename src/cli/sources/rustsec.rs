use serde::{Deserialize, Serialize};

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

    pub async fn fetch_advisories(&self, name: String, version: String ) -> Result<RustSecResponse, Box<dyn std::error::Error>> {
        let ecosystem = "crates.io".to_string();
        
        let body = OsvQuery {
            package: OsvPackageResponseData { name, ecosystem },
            version,
            commit: None,
            page_token: None,
        };
        
        let url = "https://api.osv.dev/v1/query".to_string();
        let response = self.client.post(url).json(&body).send().await?;

        let data: RustSecResponse = response.json().await?;

        Ok(data)
    }
}