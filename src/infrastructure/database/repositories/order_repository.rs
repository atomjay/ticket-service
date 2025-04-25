use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono;

use crate::domain::order::model::{CreateOrder, Order, OrderQuery, OrderView};
use crate::domain::order::repository::OrderRepository;
use crate::utils::error::AppError;

/// PostgreSQL 訂單存儲庫實現
pub struct PgOrderRepository {
    pool: PgPool,
}

impl PgOrderRepository {
    /// 創建新的 PostgreSQL 訂單存儲庫
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepository for PgOrderRepository {
    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Option<OrderView>, AppError> {
        // 使用原生 SQL 查詢
        let result = sqlx::query(
            r#"
            SELECT o.id, o.quantity, o.created_at, 
                   t.ticket_type, t.price::float8, 
                   c.title as concert_title, c.date as concert_date
            FROM orders o
            JOIN tickets t ON o.ticket_id = t.id
            JOIN concerts c ON t.concert_id = c.id
            WHERE o.id = $1 AND o.user_id = $2
            "#
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(OrderView {
                id: row.get("id"),
                quantity: row.get("quantity"),
                created_at: row.get("created_at"),
                ticket_type: row.get("ticket_type"),
                price: row.get("price"),
                concert_title: row.get("concert_title"),
                concert_date: row.get("concert_date"),
            })),
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: Uuid, query: &OrderQuery) -> Result<Vec<OrderView>, AppError> {
        // 構建基本查詢
        let mut sql = r#"
            SELECT o.id, o.quantity, o.created_at, 
                   t.ticket_type, t.price::float8, 
                   c.title as concert_title, c.date as concert_date
            FROM orders o
            JOIN tickets t ON o.ticket_id = t.id
            JOIN concerts c ON t.concert_id = c.id
            WHERE o.user_id = $1
        "#.to_string();

        // 添加過濾條件
        let mut params = Vec::new();
        params.push(format!("'{}'", user_id));

        let mut param_index = 2;

        if let Some(from) = &query.from {
            sql.push_str(&format!(" AND o.created_at::date >= ${}", param_index));
            params.push(format!("'{}'", from));
            param_index += 1;
        }

        if let Some(to) = &query.to {
            sql.push_str(&format!(" AND o.created_at::date <= ${}", param_index));
            params.push(format!("'{}'", to));
            param_index += 1;
        }

        if let Some(concert_id) = query.concert_id {
            sql.push_str(&format!(" AND t.concert_id = ${}", param_index));
            params.push(format!("'{}'", concert_id));
            param_index += 1;
        }

        // 添加排序和分頁
        sql.push_str(" ORDER BY o.created_at DESC");

        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT ${}", param_index));
            params.push(format!("{}", limit));
            param_index += 1;
        }

        if let Some(page) = query.page {
            let offset = (page - 1) * query.limit.unwrap_or(10);
            sql.push_str(&format!(" OFFSET ${}", param_index));
            params.push(format!("{}", offset));
        }

        // 執行查詢
        let result = sqlx::query(&sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let orders = result
            .iter()
            .map(|row| OrderView {
                id: row.get("id"),
                quantity: row.get("quantity"),
                created_at: row.get("created_at"),
                ticket_type: row.get("ticket_type"),
                price: row.get("price"),
                concert_title: row.get("concert_title"),
                concert_date: row.get("concert_date"),
            })
            .collect();

        Ok(orders)
    }

    async fn create(&self, user_id: Uuid, input: &CreateOrder) -> Result<Order, AppError> {
        // 使用 query! 而不是 query_as! 來手動處理 NaiveDateTime
        let record = sqlx::query!(
            r#"
            INSERT INTO orders (user_id, ticket_id, quantity)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, ticket_id, quantity, created_at
            "#,
            user_id,
            input.ticket_id,
            input.quantity
        )
        .fetch_one(&self.pool)
        .await?;

        // 手動轉換為 Order 模型
        Ok(Order {
            id: record.id,
            user_id: record.user_id,
            ticket_id: record.ticket_id,
            quantity: record.quantity,
            created_at: record.created_at.unwrap_or_else(|| chrono::Local::now().naive_local()),
        })
    }
}
