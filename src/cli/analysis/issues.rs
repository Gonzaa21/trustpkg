use crate::cli::models::metrics::{MaintenanceMetrics, SecurityMetrics, Status};
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
                recs.push(Recommendation {
                    severity: Severity::Info,
                    message: "Maintenance could not be verified (no GitHub repository or unreachable)".to_string(),
                    evidence: EvidenceKind::Text("Unknown status".to_string()),
                });
            }
            Status::Active => {}
        }
    }

    if let Some(s) = sec {
        if s.known_advisories > 0 {
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
    }

    recs
}