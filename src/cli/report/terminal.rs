use crate::cli::models::analysis::AnalysisResult;
use crate::cli::models::report::Severity;

fn render_bar(score: u32) -> String {
    let filled = (score as usize * 10) / 100;
    let empty = 10 - filled.min(10);
    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
}

pub fn render(result: &AnalysisResult) -> String {
    let mut output = String::new();

    output.push_str("== 1. PACKAGE DETAILS ==\n");
    output.push_str(&format!("Name:        {} v{}\n", result.package.name, result.package.version));
    output.push_str(&format!("Description: {}\n", result.package.description.as_deref().unwrap_or("N/A")));
    output.push_str(&format!("Repository:  {}\n", result.package.repository.as_deref().unwrap_or("N/A")));
    output.push_str(&format!("License:     {}\n", result.package.license.as_deref().unwrap_or("N/A")));
    output.push_str(&format!("Created:     {}\n", result.package.created_at));
    output.push_str(&format!("Updated:     {}\n", result.package.updated_at));
    output.push_str(&format!("Is yanked:   {}\n", result.package.yanked));
    output.push_str(&format!("Downloads:   {} (recent: {})\n", result.package.downloads, result.package.recent_downloads));
    output.push_str(&format!("Sources:     {}\n", result.metadata.sources));
    output.push('\n');

    output.push_str("== 2. SCORE ==\n");
    
    let maintenance_bar = render_bar(result.score.maintenance);
    output.push_str(&format!("Maintenance  {} {:?}\n", maintenance_bar, result.maintenance.status));

    let popularity_bar = render_bar(result.score.popularity);
    output.push_str(&format!("Popularity   {} {:3}\n", popularity_bar, result.score.popularity));

    if let Some(sec_score) = result.score.security {
        let security_bar = render_bar(sec_score);
        let security = result.security.as_ref().unwrap();
        let total_adv = security.known_advisories;
        let crit = security.critical_advisories;
        let high = security.high_advisories;
        
        output.push_str(&format!(
            "Security {} {:3} {} advisory ({} critical, {} high)\n",
            security_bar,
            sec_score,
            total_adv,
            crit,
            high
        ));
    } else {
        output.push_str("Security     [----------] N/A   (Security metric unavailable)\n");
    }

    output.push_str("----------------------------------------\n");
    
    let overall_bar = render_bar(result.score.overall);
    let partial_tag = if result.score.is_partial { " (Partial)" } else { "" };
    output.push_str(&format!("TRUST SCORE  {} {:3} / 100{}\n", overall_bar, result.score.overall, partial_tag));
    output.push('\n');

    output.push_str("== 3. FINAL REPORT ==\n");
    for rec in &result.recommendations {
        let severity_label = match rec.severity {
            Severity::Info => "INFO",
            Severity::Warning => "WARN",
            Severity::Critical => "CRITICAL",
        };
        output.push_str(&format!("[{}] {}\n", severity_label, rec.message));
    }

    output.push('\n');

    output
}