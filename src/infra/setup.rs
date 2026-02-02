use std::sync::Arc;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{
        http::app_state::AppState,
        persistence::{
            postgres::repositories::user::PostgresUserRepo, redis::token::AuthTokenCacheRepository,
        },
    },
    application::use_cases::{auth::AuthUseCase, user::UserUseCase},
    infra::{
        config::AppConfig,
        db::init_db,
        redis::init_redis,
        security::{argon2::Argon2PasswordHasher, jwt::JwtTokenProvider},
    },
};

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();
    let hasher = Argon2PasswordHasher::default();
    let token_provider = JwtTokenProvider::new(config.jwt_secret.as_str());

    let database = Arc::new(init_db().await?);
    let redis_client = init_redis(&config.redis).await?;

    let user_repository = PostgresUserRepo::new(database.clone());
    let token_cache_repository = AuthTokenCacheRepository::new(redis_client.clone());

    let user_use_case = UserUseCase::new(Arc::new(user_repository.clone()));
    let auth_use_case = AuthUseCase::new(
        Arc::new(user_repository),
        Arc::new(token_cache_repository),
        Arc::new(hasher),
        Arc::new(token_provider.clone()),
    );

    Ok(AppState {
        config: Arc::new(config),
        user_use_case: Arc::new(user_use_case),
        auth_use_case: Arc::new(auth_use_case),
        token_provider: Arc::new(token_provider),
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
