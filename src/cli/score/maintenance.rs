use chrono::{Utc, Datelike};

use crate::cli::models::metrics::{MaintenanceMetrics, Status};
use crate::cli::models::package::Package;
use crate::cli::sources::github::parse_github_url;
use crate::cli::sources::github::Github;

pub async fn calculate_maintenance(package: &Package) -> MaintenanceMetrics {
    let repository = match package.repository.as_deref() {
        Some(repo) => repo,
        None => return MaintenanceMetrics { updated_at: None, repository_archived: None, status: Status::Unknown },
    };

    let info = match parse_github_url(repository) {
        None => return MaintenanceMetrics { updated_at: None, repository_archived: None, status: Status::Unknown },
        Some((owner, repo)) => match Github::new().fetch_repo_info(&owner, &repo).await {
            Ok(info) => info,
            Err(_) => return MaintenanceMetrics { updated_at: None, repository_archived: None, status: Status::Unknown },
        },
    };

    let status = if info.archived {
        Status::Inactive
    } else {
        let current_date = Utc::now(); 
        let date = current_date.signed_duration_since(info.pushed_at);
        if date.num_days() > 365 {
            Status::Inactive
        } else {
            Status::Active
        }
    };

    MaintenanceMetrics {
        updated_at: Some(info.pushed_at),
        repository_archived: Some(info.archived),
        status,
    }
}