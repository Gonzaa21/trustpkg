use crate::cli::models::metrics::PopularityMetrics;
use crate::cli::models::package::Package;

pub fn calculate_popularity(package: &Package) -> PopularityMetrics {
    PopularityMetrics {
        downloads: package.downloads,
        recent_downloads: package.recent_downloads,
    }
}