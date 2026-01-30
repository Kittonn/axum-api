use std::env;

pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or("4001".to_string())
            .parse()
            .expect("PORT must be a number");

        Self { port }
    }
}
