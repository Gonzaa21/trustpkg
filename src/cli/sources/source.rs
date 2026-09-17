use async_trait::async_trait;
use crate::cli::models::package::Package;

#[async_trait]
pub trait Source {
    fn name(&self) -> &'static str;
    async fn fetch(&self, name: &str, version: Option<&str>) -> Result<Package, Box<dyn std::error::Error>>;
}