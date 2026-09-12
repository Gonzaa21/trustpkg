mod cli;
use crate::cli::commands::run;

#[tokio::main]
async fn main() {
    run().await;
}
