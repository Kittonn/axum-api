use std::sync::Arc;

use crate::{
    application::use_cases::{auth::AuthUseCase, user::UserUseCase},
    infra::{config::AppConfig, security::jwt::TokenProvider},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_case: Arc<UserUseCase>,
    pub auth_use_case: Arc<AuthUseCase>,
    pub token_provider: Arc<dyn TokenProvider>,
}
