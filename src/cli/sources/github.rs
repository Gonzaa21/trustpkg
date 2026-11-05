use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::fmt;

pub struct GithubRepoInfo {
    pub archived: bool,
    pub pushed_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct GithubRepoData {
    pub archived: bool,
    pub pushed_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum GithubError {
    NotFound,
    RateLimited,
    Other(String),
}

pub struct Github {
    client: reqwest::Client,
    token: Option<String>,
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GithubError::NotFound => write!(f, "Repository not found on GitHub"),
            GithubError::RateLimited => write!(f, "GitHub API rate limit exceeded"),
            GithubError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for GithubError {}

impl Github {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("trustpkg (https://github.com/Gonzaa21/trustpkg)")
            .build()
            .expect("Failed to build reqwest client");
        let token = std::env::var("GITHUB_TOKEN").ok();
        Self { client, token }
    }

    pub async fn fetch_repo_info(&self, owner: &str, repo: &str) -> Result<GithubRepoInfo, GithubError> {
        let url: String = format!("https://api.github.com/repos/{owner}/{repo}");
        
        let mut request = self.client.get(&url);
        if let Some(ref token) = self.token {
            request = request.bearer_auth(token);
        }
        
        let response = request.send().await.map_err(|e| GithubError::Other(e.to_string()))?;
        let status = response.status();
        
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(GithubError::NotFound);
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(GithubError::RateLimited);
        }

        if status == reqwest::StatusCode::FORBIDDEN {
            let headers = response.headers();
            let primary_exhausted = headers
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .is_some_and(|s| s.trim() == "0");
            let secondary_limit = headers.contains_key("retry-after");
        
            if primary_exhausted || secondary_limit {
                return Err(GithubError::RateLimited);
            }
        }

        if !status.is_success() {
            return Err(GithubError::Other(format!("GitHub API error: status {}", status)));
        }

        let data: GithubRepoData = response.json().await.map_err(|e| GithubError::Other(e.to_string()))?;

        Ok(GithubRepoInfo { 
            archived: data.archived, 
            pushed_at: data.pushed_at 
        })
    }
}

pub fn parse_github_url(url: &str) -> Option<(String, String)> {
    let parsed = url::Url::parse(url).ok()?;
    if parsed.host_str() != Some("github.com") { return None; }
    
    let mut segments = parsed.path_segments()?;

    let owner = segments.next()?; 
    let rep = segments.next()?;
    if segments.next().is_some() { return None; }

    let repo = rep.strip_suffix(".git").unwrap_or(rep);

    Some((owner.to_string(), repo.to_string()))
}