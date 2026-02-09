use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    domain::{
        entities::user::User,
        repositories::{
            error::{RepositoryError, RepositoryResult},
            user::UserRepository,
        },
    },
    infra::mssql_tiberius::TiberiusPool,
};

#[derive(Clone)]
pub struct TiberiusUserRepository {
    pool: TiberiusPool,
}

impl TiberiusUserRepository {
    pub fn new(pool: TiberiusPool) -> Self {
        Self { pool }
    }

    fn map_row(row: tiberius::Row) -> RepositoryResult<User> {
        let id_str: &str = row
            .get("id")
            .ok_or_else(|| RepositoryError::ConversionError("Missing id column".to_string()))?;

        let email: &str = row
            .get("email")
            .ok_or_else(|| RepositoryError::ConversionError("Missing email column".to_string()))?;

        let password: &str = row.get("password").ok_or_else(|| {
            RepositoryError::ConversionError("Missing password column".to_string())
        })?;

        let name: &str = row
            .get("name")
            .ok_or_else(|| RepositoryError::ConversionError("Missing name column".to_string()))?;

        let created_at_str: &str = row.get("created_at").ok_or_else(|| {
            RepositoryError::ConversionError("Missing created_at column".to_string())
        })?;

        let updated_at_str: &str = row.get("updated_at").ok_or_else(|| {
            RepositoryError::ConversionError("Missing updated_at column".to_string())
        })?;

        let id = Uuid::parse_str(id_str).map_err(|_| RepositoryError::InvalidUuidFormat)?;

        let created_at = NaiveDateTime::parse_from_str(created_at_str, "%Y-%m-%dT%H:%M:%S.%f")
            .map_err(|e| RepositoryError::ConversionError(format!("Invalid created_at: {}", e)))?
            .and_utc();

        let updated_at = NaiveDateTime::parse_from_str(updated_at_str, "%Y-%m-%dT%H:%M:%S.%f")
            .map_err(|e| RepositoryError::ConversionError(format!("Invalid updated_at: {}", e)))?
            .and_utc();

        Ok(User::from_db(
            id,
            email.to_string(),
            password.to_string(),
            name.to_string(),
            created_at,
            updated_at,
        ))
    }
}

#[async_trait]
impl UserRepository for TiberiusUserRepository {
    async fn create(&self, user: &User) -> RepositoryResult<User> {
        let mut conn = self.pool.get().await?;

        let row = conn
            .query(
                r#"
            INSERT INTO users (email, password, name)
            OUTPUT
                CAST(inserted.id AS NVARCHAR(36)) as id,
                inserted.email,
                inserted.password,
                inserted.name,
                FORMAT(inserted.created_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as created_at,
                FORMAT(inserted.updated_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as updated_at
            VALUES (@P1, @P2, @P3);
            "#,
                &[&user.email(), &user.password(), &user.name()],
            )
            .await?;
        let result = row.into_row().await?.ok_or(RepositoryError::NoRowFound)?;

        Self::map_row(result)
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let mut conn = self.pool.get().await?;

        let row = conn
            .query(
                r#"
            SELECT
                CAST(id AS NVARCHAR(36)) as id,
                email,
                password,
                name,
                FORMAT(created_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as created_at,
                FORMAT(updated_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as updated_at
            FROM users
            WHERE email = @P1
            "#,
                &[&email],
            )
            .await?;

        let row = row.into_row().await?;

        match row {
            Some(row) => Ok(Some(Self::map_row(row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_id(&self, id: &str) -> RepositoryResult<Option<User>> {
        let mut conn = self.pool.get().await?;

        let row = conn
            .query(
                r#"
            SELECT
                CAST(id AS NVARCHAR(36)) as id,
                email,
                password,
                name,
                FORMAT(created_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as created_at,
                FORMAT(updated_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as updated_at
            FROM users
            WHERE id = @P1
            "#,
                &[&id],
            )
            .await?;

        let row = row.into_row().await?;

        match row {
            Some(row) => Ok(Some(Self::map_row(row)?)),
            None => Ok(None),
        }
    }
}
