use crate::cli::models::analysis::AnalysisResult;
use crate::cli::models::report::{Severity, EvidenceKind};
use crate::cli::models::score::Verdict;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";

fn get_score_color(score: u32) -> &'static str {
    if score >= 80 {
        GREEN
    } else if score >= 50 {
        YELLOW
    } else {
        RED
    }
}

fn render_bar(score: u32) -> String {
    let filled = (score as usize * 10) / 100;
    let empty = 10 - filled.min(10);
    let color = get_score_color(score);
    format!("[{}{}{}{}]", color, "#".repeat(filled), RESET, "-".repeat(empty))
}

pub fn render(result: &AnalysisResult) -> String {
    let mut output = String::new();

    output.push_str(&format!("{}== 1. PACKAGE DETAILS =={}\n", CYAN, RESET));
    output.push_str(&format!("Name:        {}{} v{}{}\n", BOLD, result.package.name, result.package.version, RESET));
    output.push_str(&format!("Description: {}\n", result.package.description.as_deref().unwrap_or("N/A")));
    output.push_str(&format!("Repository:  {}\n", result.package.repository.as_deref().unwrap_or("N/A")));
    output.push_str(&format!("License:     {}\n", result.package.license.as_deref().unwrap_or("N/A")));
    output.push_str(&format!("Created:     {}\n", result.package.created_at));
    output.push_str(&format!("Updated:     {}\n", result.package.updated_at));
    
    let yanked_str = if result.package.yanked {
        format!("{}true{}", RED, RESET)
    } else {
        "false".to_string()
    };
    output.push_str(&format!("Is yanked:   {}\n", yanked_str));
    
    output.push_str(&format!("Downloads:   {} (recent: {})\n", result.package.downloads, result.package.recent_downloads));
    output.push_str(&format!("Sources:     {}\n", result.metadata.sources));
    output.push('\n');

    output.push_str(&format!("{}== 2. SCORE =={}\n", CYAN, RESET));
    
    let maintenance_bar = render_bar(result.score.maintenance);
    output.push_str(&format!("Maintenance  {} {:?}\n", maintenance_bar, result.maintenance.status));

    let popularity_bar = render_bar(result.score.popularity);
    output.push_str(&format!("Popularity   {} {:3}\n", popularity_bar, result.score.popularity));

    if let (Some(sec_score), Some(security)) = (result.score.security, result.security.as_ref()) {
        let security_bar = render_bar(sec_score);
        let total_adv = security.known_advisories;
        
        let crit_color = if security.critical_advisories > 0 { RED } else { RESET };
        let high_color = if security.high_advisories > 0 { YELLOW } else { RESET };
        
        output.push_str(&format!(
            "Security     {} {:3} | {} advisory ({}{} critical{}, {}{} high{})\n",
            security_bar,
            sec_score,
            total_adv,
            crit_color, security.critical_advisories, RESET,
            high_color, security.high_advisories, RESET
        ));
    } else {
        output.push_str("Security     [----------] N/A   (Security metric unavailable)\n");
    }

    output.push_str("----------------------------------------\n");
    
    let overall_bar = render_bar(result.score.overall);
    let score_color = get_score_color(result.score.overall);
    let cap_tag = match &result.score.cap_reason {
        Some(reason) => format!(" (capped: {})", reason),
        None => String::new(),
    };
    
    output.push_str(&format!(
        "{}TRUST SCORE{}  {} {}{}{} / 100{}\n", 
        BOLD, RESET, overall_bar, score_color, result.score.overall, RESET, cap_tag
    ));
    output.push('\n');
    
    let (verdict_label, verdict_color) = match result.score.verdict {
        Verdict::Trusted => ("TRUSTED", GREEN),
        Verdict::Review => ("REVIEW", YELLOW),
        Verdict::NotRecommended => ("NOT RECOMMENDED", RED),
    };
    output.push_str(&format!("VERDICT      {}{}{}\n", verdict_color, verdict_label, RESET));
    output.push('\n');
    
    output.push_str(&format!("{}== 3. FINAL REPORT =={}\n", CYAN, RESET));
    for rec in &result.recommendations {
        let (severity_label, color) = match rec.severity {
            Severity::Info => ("INFO", BLUE),
            Severity::Warning => ("WARN", YELLOW),
            Severity::Critical => ("CRITICAL", RED),
        };

        let evidence_str = match &rec.evidence {
            EvidenceKind::Date(dt) => {format!(" (Date: {})", dt.format("%Y-%m-%d"))}
            EvidenceKind::Count(c) => {format!(" (Count: {})", c)}
            EvidenceKind::Flag(f) => {format!(" (Flag: {})", f)}
            EvidenceKind::Text(t) => {format!(" ({})", t)}
        };

        output.push_str(&format!("[{}{}{}] {}{}\n", color, severity_label, RESET, rec.message, evidence_str));
    }

    output.push('\n');
    output
}