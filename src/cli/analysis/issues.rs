use crate::cli::models::metrics::{MaintenanceReason, MaintenanceMetrics, SecurityMetrics, Status};
use crate::cli::models::package::Package;
use crate::cli::models::report::{EvidenceKind, Recommendation, Severity};

pub fn detect_issues(
    package: &Package,
    maint: &MaintenanceMetrics,
    sec: Option<&SecurityMetrics>,
) -> Vec<Recommendation> {
    let mut recs = Vec::new();

    if package.yanked {
        recs.push(Recommendation {
            severity: Severity::Critical,
            message: "This version has been yanked from crates.io".to_string(),
            evidence: EvidenceKind::Flag(true),
        });
    }

    if package.license.is_none() {
        recs.push(Recommendation {
            severity: Severity::Warning,
            message: "No license declared".to_string(),
            evidence: EvidenceKind::Flag(false),
        });
    }
    
    if maint.repository_archived == Some(true) {
        recs.push(Recommendation {
            severity: Severity::Critical,
            message: "Repository is archived and no longer maintained".to_string(),
            evidence: EvidenceKind::Flag(true),
        });
    } else {
        match maint.status {
            Status::Inactive => {
                let evidence = match maint.updated_at {
                    Some(date) => EvidenceKind::Date(date),
                    None => EvidenceKind::Flag(true),
                };
                recs.push(Recommendation {
                    severity: Severity::Warning,
                    message: "No repository activity in over a year".to_string(),
                    evidence,
                });
            }
            Status::Unknown => {
                let repo = package.repository.as_deref().unwrap_or("N/A").to_string();              
                
                let (severity, message, evidence) = match maint.reason {
                    Some(MaintenanceReason::NoRepository) => (
                        Severity::Warning,
                        "No repository declared; maintenance could not be verified".to_string(),
                        EvidenceKind::Flag(false),
                    ),
                    Some(MaintenanceReason::NotGithub) => (
                        Severity::Info,
                        "Repository is not hosted on GitHub; maintenance could not be verified".to_string(),
                        EvidenceKind::Text(repo),
                    ),
                    Some(MaintenanceReason::RepositoryNotFound) => (
                        Severity::Warning,
                        "Repository not found (private, deleted or wrong URL)".to_string(),
                        EvidenceKind::Text(repo),
                    ),
                    Some(MaintenanceReason::RateLimited) => (
                        Severity::Warning,
                        "GitHub API rate limit reached; set GITHUB_TOKEN to raise it".to_string(),
                        EvidenceKind::Text(repo),
                    ),
                    Some(MaintenanceReason::RequestFailed) => (
                        Severity::Warning,
                        "Could not reach the GitHub API; maintenance could not be verified".to_string(),
                        EvidenceKind::Text(repo),
                    ),
                    None => (
                        Severity::Info,
                        "Maintenance status is unknown".to_string(),
                        EvidenceKind::Text("Unknown status".to_string()),
                    ),
                };
                
                recs.push(Recommendation { severity, message, evidence });

            }
            Status::Active => {}
        }
    }

    if let Some(s) = sec && s.known_advisories > 0 {
        let severity = if s.critical_advisories > 0 { Severity::Critical } else { Severity::Warning };
        recs.push(Recommendation {
            severity,
            message: format!(
                "{} known advisories ({} critical, {} high): {}",
                s.known_advisories, s.critical_advisories, s.high_advisories,
                s.advisory_ids.join(", ")
            ),
            evidence: EvidenceKind::Count(s.known_advisories),
        });
    }

    recs
}