use crate::infra::config::MssqlConfig;
use sqlx::{Mssql, Pool};

pub type MssqlPool = Pool<Mssql>;

pub async fn init_mssql_db(config: &MssqlConfig) -> anyhow::Result<MssqlPool> {
    let database_url = format!(
        "sqlserver://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    );
    let pool = Pool::<Mssql>::connect(&database_url).await?;

    Ok(pool)
}
