use async_trait::async_trait;
use crate::cli::models::package::Package;
use crate::cli::cache::Cache;

#[async_trait]
pub trait Source {
    fn name(&self) -> &'static str;
    async fn fetch(&self, cache: &Cache, name: &str, version: Option<&str>) -> Result<Package, Box<dyn std::error::Error>>;
}