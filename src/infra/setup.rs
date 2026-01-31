use std::sync::Arc;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{
        http::app_state::AppState, persistence::postgres::repositories::user::PostgresUserRepo,
    },
    application::use_cases::user::UserUseCase,
    infra::{config::AppConfig, db::init_db, security::argon2::Argon2PasswordHasher},
};

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();

    let database = Arc::new(init_db().await?);
    let user_repository = PostgresUserRepo::new(database.clone());
    let hasher = Argon2PasswordHasher::default();
    let user_use_case = UserUseCase::new(Arc::new(user_repository), Arc::new(hasher));

    Ok(AppState {
        config: Arc::new(config),
        user_use_case: Arc::new(user_use_case),
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let console_layer = fmt::layer().with_target(false).with_level(true).pretty();

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .try_init()
        .ok();
}
