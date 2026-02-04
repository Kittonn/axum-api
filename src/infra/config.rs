use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub jwt_secret: String,
    pub redis: RedisConfig,
    pub mssql: MssqlConfig,
    pub kafka_brokers: String,
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MssqlConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "4001".into())
            .parse()
            .expect("PORT must be a number");

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let redis = RedisConfig {
            host: env::var("REDIS_HOST").expect("REDIS_HOST must be set"),
            port: env::var("REDIS_PORT")
                .unwrap_or_else(|_| "6379".into())
                .parse()
                .expect("REDIS_PORT must be a number"),
            password: env::var("REDIS_PASSWORD").ok().filter(|v| !v.is_empty()),
        };

        let mssql = MssqlConfig {
            host: env::var("MSSQL_HOST").expect("MSSQL_HOST must be set"),
            port: env::var("MSSQL_PORT")
                .unwrap_or_else(|_| "1433".into())
                .parse()
                .expect("MSSQL_PORT must be a number"),
            database: env::var("MSSQL_DATABASE").expect("MSSQL_DATABASE must be set"),
            username: env::var("MSSQL_USERNAME").expect("MSSQL_USERNAME must be set"),
            password: env::var("MSSQL_PASSWORD").expect("MSSQL_PASSWORD must be set"),
        };

        let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".into());

        Self {
            port,
            jwt_secret,
            redis,
            mssql,
            kafka_brokers,
        }
    }
}
