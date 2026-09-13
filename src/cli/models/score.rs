#[derive(Debug)]
pub struct TrustScore {
    pub overall: u32,
    pub maintenance: u32,
    pub popularity: u32,
    pub security: u32,
}