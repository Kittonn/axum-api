use crate::domain::repositories::error::RepositoryError;

use uuid::Uuid;

#[async_trait::async_trait]
pub trait TokenCacheRepository: Send + Sync {
    async fn store_refresh_token(
        &self,
        user_id: Uuid,
        token_id: &str,
        ttl_secs: u64,
    ) -> Result<(), RepositoryError>;
    async fn get_refresh_token(&self, token_id: &str) -> Result<Option<String>, RepositoryError>;
    async fn blacklist_access_token(&self, jti: &str, ttl_secs: u64)
    -> Result<(), RepositoryError>;
    async fn is_access_token_blacklisted(&self, jti: &str) -> Result<bool, RepositoryError>;
}
