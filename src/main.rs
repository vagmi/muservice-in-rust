use anyhow::Result;
use tracing_subscriber::EnvFilter;
use mulib::greeting::greet;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    tracing::debug!("This log should only appear in debug mode");
    println!("{}", greet("confoo"));
    Ok(())
}
