use serde::Deserialize;
use chrono::{DateTime, Utc};


pub struct GithubRepoInfo {
    pub archived: bool,
    pub pushed_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct GithubRepoData {
    pub archived: bool,
    pub pushed_at: DateTime<Utc>,
}

pub struct Github {
    client: reqwest::Client,
}

impl Github {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("trustpkg (https://github.com/Gonzaa21/trustpkg)")
            .build()
            .expect("Failed to build reqwest client");
        Self { client }
    }

    pub async fn fetch_repo_info(&self, owner: &str, repo: &str) -> Result<GithubRepoInfo, Box<dyn std::error::Error>> {
        let url: String = format!("https://api.github.com/repos/{owner}/{repo}");
        let response = self.client.get(&url).send().await?;
        
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(format!("Repository '{owner}/{repo}' not found on GitHub").into());
        }

        let data: GithubRepoData = response.json().await?;

        Ok(GithubRepoInfo { 
            archived: data.archived, 
            pushed_at: data.pushed_at 
        })
    }
}

pub fn parse_github_url(url: &str) -> Option<(String, String)> {
    // extraer owner y repo de algo como "https://github.com/owner/repo"

    let parsed = url::Url::parse(url).ok()?;
    
    // devolver None si el host no es github.com o el formato no matchea
    if parsed.host_str() != Some("github.com") { return None; }
    
    let mut segments = parsed.path_segments()?;

    let owner = segments.next()?; 
    let rep = segments.next()?;
    if segments.next().is_some() { return None; }

    let repo = rep.strip_suffix(".git").unwrap_or(rep);

    Some((owner.to_string(), repo.to_string()))
}