use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::auth::model::{RegisterInput, User};
use crate::domain::auth::repository::UserRepository;
use crate::utils::error::AppError;

/// PostgreSQL 用戶存儲庫實現
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    /// 創建新的 PostgreSQL 用戶存儲庫
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        // 使用 query! 而不是 query_as! 來手動處理 Option<bool>
        let record = sqlx::query!(
            r#"
            SELECT id, email, password_hash, is_admin
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        // 手動轉換為 User 模型
        Ok(record.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
            is_admin: r.is_admin.unwrap_or(false),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        // 使用 query! 而不是 query_as! 來手動處理 Option<bool>
        let record = sqlx::query!(
            r#"
            SELECT id, email, password_hash, is_admin
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        // 手動轉換為 User 模型
        Ok(record.map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
            is_admin: r.is_admin.unwrap_or(false),
        }))
    }

    async fn create(&self, input: &RegisterInput, password_hash: &str) -> Result<(), AppError> {
        // 使用 query! 而不是 query_as! 來手動處理 Option<bool>
        sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, is_admin)
            VALUES ($1, $2, false)
            "#,
            input.email,
            password_hash
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let exists = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?
        .exists
        .unwrap_or(false);

        Ok(exists)
    }
}
