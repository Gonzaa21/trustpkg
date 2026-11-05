use chrono::Utc;
use crate::cli::models::metrics::{MaintenanceMetrics, Status, MaintenanceReason};
use crate::cli::models::package::Package;
use crate::cli::sources::github::{GithubError, parse_github_url};
use crate::cli::sources::github::Github;

fn unknown(reason: MaintenanceReason) -> MaintenanceMetrics {
    MaintenanceMetrics {
        updated_at: None,
        repository_archived: None,
        status: Status::Unknown,
        reason: Some(reason),
    }
}

pub async fn calculate_maintenance(package: &Package) -> MaintenanceMetrics {
    let repository = match package.repository.as_deref() {
        Some(repo) => repo,
        None => return unknown(MaintenanceReason::NoRepository),
    };

    let (owner, repo) = match parse_github_url(repository) {
        Some(parsed) => parsed,
        None => return unknown(MaintenanceReason::NotGithub),
    };

    let info = match Github::new().fetch_repo_info(&owner, &repo).await {
        Ok(info) => info,
        Err(GithubError::NotFound) => return unknown(MaintenanceReason::RepositoryNotFound),
        Err(GithubError::RateLimited) => return unknown(MaintenanceReason::RateLimited),
        Err(GithubError::Other(_)) => return unknown(MaintenanceReason::RequestFailed),
    };
    
    let status = if info.archived || Utc::now().signed_duration_since(info.pushed_at).num_days() > 365 {
        Status::Inactive
    } else {
        Status::Active
    };

    MaintenanceMetrics {
        updated_at: Some(info.pushed_at),
        repository_archived: Some(info.archived),
        status,
        reason: None,
    }
}