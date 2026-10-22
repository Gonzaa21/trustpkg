use crate::cli::models::metrics::{Status, SecurityMetrics, MaintenanceMetrics, PopularityMetrics};
use crate::cli::models::score::TrustScore;
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

pub fn calculate_score (sec: Option<u32>, maint: &MaintenanceMetrics, pop: &PopularityMetrics,) -> TrustScore {
    let maintenance = score_maintenance(maint);
    let popularity = score_popularity(pop);

    let mut weighted_sum = 0.0;
    let mut total_weight = 0.0;

    let security_score = if let Some(s) = sec {
        weighted_sum += s as f64 * WEIGHT_SECURITY;
        total_weight += WEIGHT_SECURITY;
        Some(s)
    } else {
        None
    };

    weighted_sum += maintenance as f64 * WEIGHT_MAINTENANCE;
    total_weight += WEIGHT_MAINTENANCE;

    weighted_sum += popularity as f64 * WEIGHT_POPULARITY;
    total_weight += WEIGHT_POPULARITY;

    let overall_f = if total_weight > 0.0 {
        weighted_sum / total_weight
    } else {
        0.0
    };

    TrustScore {
        overall: overall_f.round() as u32,
        security: security_score,
        maintenance,
        popularity,
        is_partial: sec.is_none()
    }
}