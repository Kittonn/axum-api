use anyhow::Result;
use redis::{Client, aio::ConnectionManager};

use crate::infra::config::RedisConfig;

pub async fn init_redis(cfg: &RedisConfig) -> Result<ConnectionManager> {
    let url = match &cfg.password {
        Some(pw) => format!("redis://:{}@{}:{}", pw, cfg.host, cfg.port),
        None => format!("redis://{}:{}", cfg.host, cfg.port),
    };

    let client = Client::open(url)?;
    let conn = client.get_connection_manager().await?;

    Ok(conn)
}
