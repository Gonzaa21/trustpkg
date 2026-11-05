use crate::cli::models::metrics::{Status, SecurityMetrics, MaintenanceMetrics, PopularityMetrics};
use crate::cli::models::{score::{TrustScore, Verdict}, package::Package};
use chrono::Utc;

const WEIGHT_SECURITY: f64 = 0.50;
const WEIGHT_MAINTENANCE: f64 = 0.30;
const WEIGHT_POPULARITY: f64 = 0.20;

pub fn score_security(metrics: &SecurityMetrics) -> u32 {
    let mut score = 100_u32;
    score = score.saturating_sub(metrics.critical_advisories * 40);
    score = score.saturating_sub(metrics.high_advisories * 20);
    
    let other_advisories = metrics.known_advisories
        .saturating_sub(metrics.critical_advisories + metrics.high_advisories);
    score = score.saturating_sub(other_advisories * 5);

    score
}

pub fn score_maintenance(metrics: &MaintenanceMetrics) -> u32 {
    let base_score: u32 = match metrics.status {
        Status::Active => 100,
        Status::Unknown => 60,
        Status::Inactive => 40,
    };

    let mut score = base_score;

    if metrics.status == Status::Active && let Some(updated_at) = metrics.updated_at {
            let now = Utc::now();
            let duration = now.signed_duration_since(updated_at);
            let days = duration.num_days();

            if days > 365 {
                score = score.saturating_sub(30);
            } else if days > 180 {
                score = score.saturating_sub(15);
            }
        }

    score
}

pub fn score_popularity(metrics: &PopularityMetrics) -> u32 {
    let effective_downloads = metrics.downloads as f64;

    if effective_downloads <= 0.0 {
        return 0;
    }

    let log_val = effective_downloads.log10();
    let calculated = (log_val / 8.0) * 100.0;

    calculated.clamp(0.0, 100.0) as u32
}

fn score_cap(
    package: &Package,
    maint: &MaintenanceMetrics,
    sec: Option<&SecurityMetrics>,
) -> Option<(u32, &'static str)> {
    let mut caps: Vec<(u32, &'static str)> = Vec::new();

    if package.yanked {
        caps.push((20, "yanked version"));
    }
    if sec.is_some_and(|s| s.critical_advisories > 0) {
        caps.push((30, "critical advisory"));
    }
    if maint.repository_archived == Some(true) {
        caps.push((40, "archived repository"));
    }
    if maint.status == Status::Inactive {
        caps.push((60, "inactive repository"));
    }

    caps.into_iter().min_by_key(|(limit, _)| *limit)
}

pub fn calculate_score(
    package: &Package,
    sec: Option<&SecurityMetrics>,
    maint: &MaintenanceMetrics,
    pop: &PopularityMetrics,
) -> TrustScore {
    let maintenance = score_maintenance(maint);
    let popularity = score_popularity(pop);
    let security_score = sec.map(score_security);

    let mut weighted_sum = maintenance as f64 * WEIGHT_MAINTENANCE
        + popularity as f64 * WEIGHT_POPULARITY;
    let mut total_weight = WEIGHT_MAINTENANCE + WEIGHT_POPULARITY;

    if let Some(s) = security_score {
        weighted_sum += s as f64 * WEIGHT_SECURITY;
        total_weight += WEIGHT_SECURITY;
    }

    let uncapped = (weighted_sum / total_weight).round() as u32;

    let cap = score_cap(package, maint, sec);
    let overall = match cap {
        Some((limit, _)) => uncapped.min(limit),
        None => uncapped,
    };
    
    let cap_reason = cap
        .filter(|(limit, _)| uncapped > *limit)
        .map(|(_, reason)| reason.to_string());

    TrustScore {
        overall,
        security: security_score,
        maintenance,
        popularity,
        is_partial: sec.is_none(),
        cap_reason,
        verdict: Verdict::from_score(overall),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::cli::models::metrics::MaintenanceReason;
//     use crate::cli::analysis::issues::detect_issues;

//     fn dummy_package() -> Package {
//         Package {
//             name: "dummy".to_string(),
//             version: "1.0.0".to_string(),
//             description: None,
//             repository: None,
//             created_at: Utc::now(),
//             downloads: 1_000_000_000,
//             recent_downloads: 100_000,
//             updated_at: Utc::now(),
//             keywords: vec![],
//             license: Some("MIT".to_string()),
//             yanked: false,
//         }
//     }

//     fn dummy_security_metrics() -> SecurityMetrics {
//         SecurityMetrics {
//             known_advisories: 0,
//             critical_advisories: 0,
//             high_advisories: 0,
//             advisory_ids: vec![],
//         }
//     }

//     fn dummy_maintenance_metrics() -> MaintenanceMetrics {
//         MaintenanceMetrics {
//             status: Status::Active,
//             updated_at: Some(Utc::now()),
//             repository_archived: Some(false),
//             reason: Some(MaintenanceReason::RepositoryNotFound),
//         }
//     }

//     // Popularidad máxima: 1e9 descargas => log10 = 9 => se topa en 100
//     fn dummy_popularity_metrics() -> PopularityMetrics {
//         PopularityMetrics {
//             downloads: 1_000_000_000,
//             recent_downloads: 100_000,
//         }
//     }

//     #[test]
//     fn test_score_security_bounds() {
//         assert_eq!(score_security(&dummy_security_metrics()), 100);

//         let sec_heavy = SecurityMetrics {
//             known_advisories: 5,
//             critical_advisories: 5,
//             high_advisories: 0,
//             advisory_ids: vec![],
//         };
//         assert_eq!(score_security(&sec_heavy), 0);
//     }

//     #[test]
//     fn test_score_cap_precedence() {
//         let maint = MaintenanceMetrics {
//             status: Status::Inactive,
//             updated_at: Some(Utc::now() - chrono::Duration::days(400)),
//             repository_archived: Some(true),
//             reason: None
//         };
//         let sec = dummy_security_metrics();

//         let cap = score_cap(&dummy_package(), &maint, Some(&sec));
//         assert_eq!(cap, Some((40, "archived repository")));
//     }

//     #[test]
//     fn test_verdict_from_score_edges() {
//         assert_eq!(Verdict::from_score(49), Verdict::NotRecommended);
//         assert_eq!(Verdict::from_score(50), Verdict::Review);
//         assert_eq!(Verdict::from_score(79), Verdict::Review);
//         assert_eq!(Verdict::from_score(80), Verdict::Trusted);
//     }

//     #[test]
//     fn test_calculate_score_without_security() {
//         let maint = dummy_maintenance_metrics();
//         let pop = dummy_popularity_metrics();

//         let trust_score = calculate_score(&dummy_package(), None, &maint, &pop);

//         assert!(trust_score.is_partial);
//         assert!(trust_score.security.is_none());
//         assert_eq!(trust_score.overall, 100);
//     }

//     #[test]
//     fn test_cap_reason_reported_only_when_active() {
//         let pop = dummy_popularity_metrics();
//         let sec = dummy_security_metrics();

//         let maint_archived = MaintenanceMetrics {
//             repository_archived: Some(true),
//             ..dummy_maintenance_metrics()
//         };
//         let capped = calculate_score(&dummy_package(), Some(&sec), &maint_archived, &pop);
//         assert_eq!(capped.overall, 40);
//         assert!(capped.cap_reason.is_some());

//         let normal = calculate_score(&dummy_package(), Some(&sec), &dummy_maintenance_metrics(), &pop);
//         assert_eq!(normal.overall, 100);
//         assert!(normal.cap_reason.is_none());
//     }

//     #[test]
//     fn test_cap_reason_none_when_score_already_below_cap() {
//         // Inactivo (tope 60), pero con seguridad y popularidad bajas el score ya queda por debajo
//         let maint = MaintenanceMetrics {
//             status: Status::Inactive,
//             updated_at: None,
//             repository_archived: Some(false),
//             reason: None
//         };
//         let sec = SecurityMetrics {
//             known_advisories: 5,
//             critical_advisories: 0,
//             high_advisories: 5,
//             advisory_ids: vec![],
//         };
//         let pop = PopularityMetrics { downloads: 10, recent_downloads: 0 };

//         let result = calculate_score(&dummy_package(), Some(&sec), &maint, &pop);

//         assert!(result.overall < 60);
//         assert!(result.cap_reason.is_none());
//     }

//     #[test]
//     fn test_detect_issues_rate_limited() {
//         let maint = MaintenanceMetrics {
//             status: Status::Unknown,
//             updated_at: None,
//             repository_archived: None,
//             reason: None
//         };
//         let package = dummy_package();
//         let sec = dummy_security_metrics();
        
//         let issues = detect_issues(&package, &maint, Some(&sec));
        
//         let found_github_token_warning = issues
//             .iter()
//             .any(|issue| format!("{issue:?}").contains("GITHUB_TOKEN"));
//         assert!(found_github_token_warning, "El warning por RateLimited debe mencionar GITHUB_TOKEN");
//     }
// }