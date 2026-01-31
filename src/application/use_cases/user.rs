use std::sync::Arc;

use crate::domain::repositories::user::UserRepository;

pub struct UserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl UserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}
