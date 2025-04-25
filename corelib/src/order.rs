use axum::{
    extract::{Json, State, Path, Query},
    http::StatusCode,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use validator::Validate;
use utoipa::{ToSchema, IntoParams};
use crate::auth::AuthClaims;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateOrder {
    pub ticket_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

/// 建立新的訂單
#[utoipa::path(
    post,
    path = "/api/orders",
    request_body = CreateOrder,
    responses(
        (status = 201)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_order(
    State(pool): State<PgPool>,
    AuthClaims(claims): AuthClaims,
    Json(input): Json<CreateOrder>,
) -> StatusCode {
    let user_id = claims.sub.parse::<Uuid>().unwrap();
    let mut tx = pool.begin().await.unwrap();

    let ticket = sqlx::query!(
        "SELECT stock FROM tickets WHERE id = $1",
        input.ticket_id
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    if ticket.stock < input.quantity {
        return StatusCode::BAD_REQUEST;
    }

    sqlx::query!(
        "UPDATE tickets SET stock = stock - $1 WHERE id = $2",
        input.quantity,
        input.ticket_id
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO orders (user_id, ticket_id, quantity) VALUES ($1, $2, $3)",
        user_id,
        input.ticket_id,
        input.quantity
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();
    StatusCode::CREATED
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct OrderQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub concert_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct OrderView {
    pub id: Uuid,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub ticket_type: String,
    pub price: f64,
    pub concert_title: String,
    pub concert_date: NaiveDateTime,
}

/// 列出用戶的所有訂單
#[utoipa::path(
    get,
    path = "/api/orders",
    params(OrderQuery),
    responses(
        (status = 200, body = Vec<OrderView>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn list_orders(
    State(pool): State<PgPool>,
    AuthClaims(claims): AuthClaims,
    Query(query): Query<OrderQuery>,
) -> Result<Json<Vec<OrderView>>, StatusCode> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).min(100);
    let offset = (page - 1) * limit;
    let user_id = claims.sub.parse::<Uuid>().unwrap();

    // 使用原生 SQL 查詢，避免 sqlx::query_as! 宏的類型轉換問題
    let query_sql = if let Some(concert_id) = query.concert_id {
        // 按演唱會篩選
        let result = sqlx::query(
            r#"
            SELECT o.id, o.quantity, o.created_at,
                   t.ticket_type, t.price::float8,
                   c.title as concert_title, c.date as concert_date
            FROM orders o
            JOIN tickets t ON o.ticket_id = t.id
            JOIN concerts c ON t.concert_id = c.id
            WHERE o.user_id = $1 AND t.concert_id = $2
            ORDER BY o.created_at DESC
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(user_id)
        .bind(concert_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await;

        result
    } else if let (Some(from), Some(to)) = (query.from, query.to) {
        // 按日期範圍篩選
        let from = from.and_hms_opt(0, 0, 0).unwrap();
        let to = to.and_hms_opt(23, 59, 59).unwrap();
        
        let result = sqlx::query(
            r#"
            SELECT o.id, o.quantity, o.created_at,
                   t.ticket_type, t.price::float8,
                   c.title as concert_title, c.date as concert_date
            FROM orders o
            JOIN tickets t ON o.ticket_id = t.id
            JOIN concerts c ON t.concert_id = c.id
            WHERE o.user_id = $1 AND o.created_at BETWEEN $2 AND $3
            ORDER BY o.created_at DESC
            LIMIT $4 OFFSET $5
            "#
        )
        .bind(user_id)
        .bind(from)
        .bind(to)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await;

        result
    } else {
        // 不篩選
        let result = sqlx::query(
            r#"
            SELECT o.id, o.quantity, o.created_at,
                   t.ticket_type, t.price::float8,
                   c.title as concert_title, c.date as concert_date
            FROM orders o
            JOIN tickets t ON o.ticket_id = t.id
            JOIN concerts c ON t.concert_id = c.id
            WHERE o.user_id = $1
            ORDER BY o.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(user_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await;

        result
    };

    match query_sql {
        Ok(rows) => {
            let orders: Vec<OrderView> = rows
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
            Ok(Json(orders))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 獲取特定訂單詳情
#[utoipa::path(
    get,
    path = "/api/orders/{id}",
    params(
        ("id", Path, description = "訂單 ID")
    ),
    responses(
        (status = 200, body = OrderView),
        (status = 404)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_order_by_id(
    Path(order_id): Path<Uuid>,
    State(pool): State<PgPool>,
    AuthClaims(claims): AuthClaims,
) -> Result<Json<OrderView>, StatusCode> {
    let user_id = claims.sub.parse::<Uuid>().unwrap();

    // 使用原生 SQL 查詢，避免 sqlx::query_as! 宏的類型轉換問題
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
    .bind(order_id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Some(row) => {
            let order = OrderView {
                id: row.get("id"),
                quantity: row.get("quantity"),
                created_at: row.get("created_at"),
                ticket_type: row.get("ticket_type"),
                price: row.get("price"),
                concert_title: row.get("concert_title"),
                concert_date: row.get("concert_date"),
            };
            Ok(Json(order))
        },
        None => Err(StatusCode::NOT_FOUND),
    }
}
