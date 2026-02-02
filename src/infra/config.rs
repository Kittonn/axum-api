use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub jwt_secret: String,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
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

        Self {
            port,
            jwt_secret,
            redis,
        }
    }
}
