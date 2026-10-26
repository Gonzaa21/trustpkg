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

    if metrics.status == Status::Active {
        if let Some(updated_at) = metrics.updated_at {
            let now = Utc::now();
            let duration = now.signed_duration_since(updated_at);
            let days = duration.num_days();

            if days > 365 {
                score = score.saturating_sub(30);
            } else if days > 180 {
                score = score.saturating_sub(15);
            }
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