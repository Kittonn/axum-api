use std::time::Duration;

use anyhow::Result;
use sea_orm::{ConnectOptions, Database};

const DATABASE_URL: &str = "";

#[tokio::main]
async fn main() -> Result<()> {
    let mut opt = ConnectOptions::new(DATABASE_URL);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await?;

    Ok(())
}
