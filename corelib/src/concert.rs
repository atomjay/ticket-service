use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use utoipa::{ToSchema, IntoParams};

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateConcert {
    #[validate(length(min = 3))]
    pub title: String,
    pub date: NaiveDateTime,
    #[validate(length(min = 2))]
    pub venue: String,
}

/// 建立新的演唱會
#[utoipa::path(
    post,
    path = "/api/concerts",
    request_body = CreateConcert,
    responses(
        (status = 201)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_concert(
    State(pool): State<PgPool>,
    Json(input): Json<CreateConcert>,
) -> StatusCode {
    let result = sqlx::query!(
        "INSERT INTO concerts (title, date, venue) VALUES ($1, $2, $3)",
        input.title,
        input.date,
        input.venue
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
