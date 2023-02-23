use anyhow::Result;
use mulib::server::start_server;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    tracing::debug!("This log should only appear in debug mode");

    start_server().await?;
    Ok(())
}
