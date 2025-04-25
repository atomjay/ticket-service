use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::concert::model::{Concert, CreateConcert};
use crate::domain::concert::repository::ConcertRepository;
use crate::utils::error::AppError;

/// PostgreSQL 演唱會存儲庫實現
pub struct PgConcertRepository {
    pool: PgPool,
}

impl PgConcertRepository {
    /// 創建新的 PostgreSQL 演唱會存儲庫
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConcertRepository for PgConcertRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Concert>, AppError> {
        // 使用 query! 而不是 query_as! 來手動處理
        let record = sqlx::query!(
            r#"
            SELECT id, title, venue, date
            FROM concerts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        // 手動轉換為 Concert 模型
        Ok(record.map(|r| Concert {
            id: r.id,
            title: r.title,
            artist: "未知藝術家".to_string(), // 暫時使用預設值
            venue: r.venue,
            date: r.date,
        }))
    }

    async fn find_all(&self) -> Result<Vec<Concert>, AppError> {
        // 使用 query! 而不是 query_as! 來手動處理
        let records = sqlx::query!(
            r#"
            SELECT id, title, venue, date
            FROM concerts
            ORDER BY date
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // 手動轉換為 Concert 模型
        let concerts = records
            .into_iter()
            .map(|r| Concert {
                id: r.id,
                title: r.title,
                artist: "未知藝術家".to_string(), // 暫時使用預設值
                venue: r.venue,
                date: r.date,
            })
            .collect();

        Ok(concerts)
    }

    async fn create(&self, input: &CreateConcert) -> Result<Concert, AppError> {
        // 使用 query! 而不是 query_as! 來手動處理
        let record = sqlx::query!(
            r#"
            INSERT INTO concerts (title, venue, date)
            VALUES ($1, $2, $3)
            RETURNING id, title, venue, date
            "#,
            input.title,
            input.venue,
            input.date
        )
        .fetch_one(&self.pool)
        .await?;

        // 手動轉換為 Concert 模型
        Ok(Concert {
            id: record.id,
            title: record.title,
            artist: input.artist.clone(), // 使用輸入的藝術家名稱，但不存儲到資料庫
            venue: record.venue,
            date: record.date,
        })
    }
}
