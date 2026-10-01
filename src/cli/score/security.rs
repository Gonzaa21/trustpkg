use crate::cli::models::metrics::{SecurityMetrics, VulnSeverity};
use crate::cli::models::package::Package;
use crate::cli::sources::rustsec::{Rustsec, VulnsData};

pub async fn calculate_security(package: &Package) -> Result<SecurityMetrics, Box<dyn std::error::Error>> {

    let resp = Rustsec::new()
        .fetch_advisories(package.name.clone(), package.version.clone())
        .await?;

    fn highest_severity(vuln: &VulnsData) -> VulnSeverity {
        vuln.affected
            .iter()
            .map(|aff| {
                aff.ecosystem_specific
                    .as_ref()
                    .and_then(|es| es.severity.as_deref())
                    .map(VulnSeverity::from)
                    .unwrap_or(VulnSeverity::Unknown)
            })
            .max()
            .unwrap_or(VulnSeverity::Unknown)
    }

    let critical_advisories = resp.vulns.iter()
        .filter(|v| highest_severity(v) == VulnSeverity::Critical)
        .count();

    let high_advisories = resp.vulns.iter()
        .filter(|v| highest_severity(v) == VulnSeverity::High)
        .count();

    Ok(SecurityMetrics {
        known_advisories: u32::try_from(resp.vulns.len())?,
        critical_advisories: u32::try_from(critical_advisories)?,
        high_advisories: u32::try_from(high_advisories)?,
    })
}