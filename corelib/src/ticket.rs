use axum::{extract::{Json, State, Query}, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::BigDecimal, Row};
use uuid::Uuid;
use validator::Validate;
use utoipa::{ToSchema, IntoParams};
use std::str::FromStr;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateTicket {
    pub concert_id: Uuid,
    pub ticket_type: String,
    pub price: f64,
    pub stock: i32,
}

#[derive(Serialize, ToSchema)]
pub struct Ticket {
    pub id: Uuid,
    pub concert_id: Uuid,
    pub ticket_type: String,
    pub price: f64,
    pub stock: i32,
}

/// 建立新的票券
#[utoipa::path(
    post,
    path = "/api/tickets",
    request_body = CreateTicket,
    responses(
        (status = 201)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_ticket(
    State(pool): State<PgPool>,
    crate::auth::AuthClaims(claims): crate::auth::AuthClaims,
    Json(input): Json<CreateTicket>,
) -> StatusCode {
    if !claims.admin {
        return StatusCode::FORBIDDEN;
    }

    // 將 f64 轉換為 BigDecimal
    let price_decimal = BigDecimal::from_str(&input.price.to_string())
        .unwrap_or_else(|_| BigDecimal::from_str("0").unwrap());

    let result = sqlx::query!(
        "INSERT INTO tickets (concert_id, ticket_type, price, stock) VALUES ($1, $2, $3, $4)",
        input.concert_id,
        input.ticket_type,
        price_decimal,
        input.stock
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Deserialize, IntoParams)]
pub struct TicketQuery {
    pub concert_id: Uuid,
}

/// 列出指定演唱會的票券
#[utoipa::path(
    get,
    path = "/api/tickets",
    params(TicketQuery),
    responses(
        (status = 200, body = Vec<Ticket>)
    )
)]
pub async fn list_tickets(
    State(pool): State<PgPool>,
    Query(query): Query<TicketQuery>,
) -> Json<Vec<Ticket>> {
    // 使用原生 SQL 查詢，避免 sqlx::query_as! 宏的類型轉換問題
    let result = sqlx::query(
        r#"
        SELECT id, concert_id, ticket_type, price::float8, stock 
        FROM tickets 
        WHERE concert_id = $1
        "#
    )
    .bind(query.concert_id)
    .fetch_all(&pool)
    .await
    .unwrap();

    let tickets: Vec<Ticket> = result
        .iter()
        .map(|row| Ticket {
            id: row.get("id"),
            concert_id: row.get("concert_id"),
            ticket_type: row.get("ticket_type"),
            price: row.get("price"),
            stock: row.get("stock"),
        })
        .collect();

    Json(tickets)
}
