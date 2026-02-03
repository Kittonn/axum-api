use async_trait::async_trait;

use crate::{
    adapters::persistence::mssql::entities::user::UserEntity,
    domain::{
        entities::user::User,
        repositories::{error::RepositoryError, user::UserRepository},
    },
    infra::mssql::MssqlPool,
};

#[derive(Clone)]
pub struct MssqlUserRepo {
    pool: MssqlPool,
}

impl MssqlUserRepo {
    pub fn new(pool: MssqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for MssqlUserRepo {
    async fn create(&self, user: &User) -> Result<User, RepositoryError> {
        let row = sqlx::query_as::<_, UserEntity>(
            r#"
            INSERT INTO users (email, password, name)
            OUTPUT
                CAST(inserted.id AS NVARCHAR(36)) as id,
                inserted.email,
                inserted.password,
                inserted.name,
                FORMAT(inserted.created_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as created_at,
                FORMAT(inserted.updated_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as updated_at
            VALUES (@p1, @p2, @p3);
            "#,
        )
        .bind(user.email())
        .bind(user.password())
        .bind(user.name())
        .fetch_one(&self.pool)
        .await?;

        Ok(row.to_domain())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query_as::<_, UserEntity>(
            r#"
            SELECT
                CAST(id AS NVARCHAR(36)) as id,
                email,
                password,
                name,
                FORMAT(created_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as created_at,
                FORMAT(updated_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as updated_at
            FROM users
            WHERE email = @p1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        println!("row: {:?}", row);

        match row {
            Some(entity) => Ok(Some(entity.to_domain())),
            None => Ok(None),
        }
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query_as::<_, UserEntity>(
            r#"
            SELECT
                CAST(id AS NVARCHAR(36)) as id,
                email,
                password,
                name,
                FORMAT(created_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as created_at,
                FORMAT(updated_at, 'yyyy-MM-ddTHH:mm:ss.ffffff') as updated_at
            FROM users
            WHERE id = @p1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(entity) => Ok(Some(entity.to_domain())),
            None => Ok(None),
        }
    }
}
