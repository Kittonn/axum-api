use std::env;

pub struct AppConfig {
    pub port: u16,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or("4001".to_string())
            .parse()
            .expect("PORT must be a number");

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Self { port, jwt_secret }
    }
}
