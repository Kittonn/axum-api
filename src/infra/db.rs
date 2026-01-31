use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};
use tracing::info;

pub async fn init_db() -> anyhow::Result<DatabaseConnection> {
    let database_url = env::var("DATABASE_URL")?;
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await?;

    info!("Database connected");

    Ok(db)
}
