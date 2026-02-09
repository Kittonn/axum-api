use crate::domain::repositories::{error::RepositoryResult, token_cache::TokenCacheRepository};
use redis::{AsyncCommands, aio::ConnectionManager};
use uuid::Uuid;

pub struct AuthTokenCacheRepository {
    conn: ConnectionManager,
}

impl AuthTokenCacheRepository {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    fn refresh_key(token_id: &str) -> String {
        format!("auth:refresh:{token_id}")
    }

    fn blacklist_key(jti: &str) -> String {
        format!("auth:blacklist:{jti}")
    }
}

#[async_trait::async_trait]
impl TokenCacheRepository for AuthTokenCacheRepository {
    async fn store_refresh_token(
        &self,
        user_id: Uuid,
        token_id: &str,
        ttl_secs: u64,
    ) -> RepositoryResult<()> {
        let key = Self::refresh_key(token_id);
        let mut conn = self.conn.clone();
        let _: () = conn.set_ex(key, user_id.to_string(), ttl_secs).await?;

        Ok(())
    }

    async fn get_refresh_token(&self, token_id: &str) -> RepositoryResult<Option<String>> {
        let mut conn = self.conn.clone();

        let key = Self::refresh_key(token_id);
        let value = conn.get(key).await?;

        Ok(value)
    }

    async fn blacklist_access_token(&self, jti: &str, ttl_secs: u64) -> RepositoryResult<()> {
        let mut conn = self.conn.clone();

        let key = Self::blacklist_key(jti);
        let _: () = conn.set_ex(key, true, ttl_secs).await?;

        Ok(())
    }

    async fn is_access_token_blacklisted(&self, jti: &str) -> RepositoryResult<bool> {
        let mut conn = self.conn.clone();

        let key = Self::blacklist_key(jti);
        let exists = conn.exists(key).await?;

        Ok(exists)
    }
}
