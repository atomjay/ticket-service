use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::str::FromStr;
use sqlx::types::BigDecimal;
use uuid::Uuid;

use crate::domain::ticket::model::{CreateTicket, Ticket};
use crate::domain::ticket::repository::TicketRepository;
use crate::utils::error::AppError;

/// PostgreSQL 票券存儲庫實現
pub struct PgTicketRepository {
    pool: PgPool,
}

impl PgTicketRepository {
    /// 創建新的 PostgreSQL 票券存儲庫
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TicketRepository for PgTicketRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Ticket>, AppError> {
        // 使用原生 SQL 查詢，避免 sqlx::query_as! 宏的類型轉換問題
        let result = sqlx::query(
            r#"
            SELECT id, concert_id, ticket_type, price::float8, stock 
            FROM tickets 
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(Ticket {
                id: row.get("id"),
                concert_id: row.get("concert_id"),
                ticket_type: row.get("ticket_type"),
                price: row.get("price"),
                stock: row.get("stock"),
            })),
            None => Ok(None),
        }
    }

    async fn find_by_concert_id(&self, concert_id: Uuid) -> Result<Vec<Ticket>, AppError> {
        // 使用原生 SQL 查詢
        let result = sqlx::query(
            r#"
            SELECT id, concert_id, ticket_type, price::float8, stock 
            FROM tickets 
            WHERE concert_id = $1
            "#
        )
        .bind(concert_id)
        .fetch_all(&self.pool)
        .await?;

        let tickets = result
            .iter()
            .map(|row| Ticket {
                id: row.get("id"),
                concert_id: row.get("concert_id"),
                ticket_type: row.get("ticket_type"),
                price: row.get("price"),
                stock: row.get("stock"),
            })
            .collect();

        Ok(tickets)
    }

    async fn create(&self, input: &CreateTicket) -> Result<Ticket, AppError> {
        // 將 f64 轉換為 BigDecimal
        let price_decimal = BigDecimal::from_str(&input.price.to_string())
            .unwrap_or_else(|_| BigDecimal::from_str("0").unwrap());

        // 使用原生 SQL 查詢
        let result = sqlx::query(
            r#"
            INSERT INTO tickets (concert_id, ticket_type, price, stock)
            VALUES ($1, $2, $3, $4)
            RETURNING id, concert_id, ticket_type, price::float8, stock
            "#
        )
        .bind(input.concert_id)
        .bind(&input.ticket_type)
        .bind(price_decimal)
        .bind(input.stock)
        .fetch_one(&self.pool)
        .await?;

        let ticket = Ticket {
            id: result.get("id"),
            concert_id: result.get("concert_id"),
            ticket_type: result.get("ticket_type"),
            price: result.get("price"),
            stock: result.get("stock"),
        };

        Ok(ticket)
    }

    async fn update_stock(&self, id: Uuid, quantity: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE tickets
            SET stock = stock - $1
            WHERE id = $2
            "#,
            quantity,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
