use sqlx::{Mssql, Pool};

pub type MssqlPool = Pool<Mssql>;

pub async fn init_mssql_db(database_url: &str) -> anyhow::Result<MssqlPool> {
    let pool = Pool::<Mssql>::connect(database_url).await?;

    Ok(pool)
}
