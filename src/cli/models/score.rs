use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TrustScore {
    pub overall: u32,
    pub maintenance: u32,
    pub popularity: u32,
    pub security: Option<u32>,
    pub is_partial: bool,
}