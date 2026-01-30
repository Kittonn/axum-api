use crate::domain::entities::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), String>;
    async fn find_by_email(&self, email: &str) -> Option<User>;
}
