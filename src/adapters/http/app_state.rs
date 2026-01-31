use std::sync::Arc;

use crate::{application::use_cases::user::UserUseCase, infra::config::AppConfig};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_case: Arc<UserUseCase>,
}
