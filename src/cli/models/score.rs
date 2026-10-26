use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub enum Verdict {
    Trusted,
    Review,
    NotRecommended,
}

impl Verdict {
    pub fn from_score(score: u32) -> Self {
        if score >= 80 {
            Verdict::Trusted
        } else if score >= 50 {
            Verdict::Review
        } else {
            Verdict::NotRecommended
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrustScore {
    pub overall: u32,
    pub security: Option<u32>,
    pub maintenance: u32,
    pub popularity: u32,
    pub is_partial: bool,
    pub cap_reason: Option<String>,
    pub verdict: Verdict,
}