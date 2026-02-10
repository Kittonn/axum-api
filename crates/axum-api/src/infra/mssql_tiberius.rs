use std::time::Duration;

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::{AuthMethod, Config};

use crate::infra::config::MssqlConfig;

pub type TiberiusPool = Pool<ConnectionManager>;

pub async fn init_mssql_tiberius(cfg: &MssqlConfig) -> anyhow::Result<TiberiusPool> {
    let mut config = Config::new();
    config.host(&cfg.host);
    config.port(cfg.port);
    config.database(&cfg.database);
    config.authentication(AuthMethod::sql_server(
        cfg.username.clone(),
        cfg.password.clone(),
    ));
    config.trust_cert();
    config.encryption(tiberius::EncryptionLevel::Off);

    let manager = ConnectionManager::new(config);
    let pool = Pool::builder()
        .max_size(100)
        .connection_timeout(Duration::from_secs(300))
        .build(manager)
        .await?;

    Ok(pool)
}
