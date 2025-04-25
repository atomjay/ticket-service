use axum::{
    extract::{Json, State},
};

use crate::api::middleware::auth::AdminUser;
use crate::api::routes::AppState;
use crate::domain::concert::model::{Concert, CreateConcert};
use crate::utils::error::AppError;

/// 創建音樂會處理程序
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/concerts",
    request_body = CreateConcert,
    responses(
        (status = 200, description = "成功創建音樂會", body = Concert),
        (status = 401, description = "未認證"),
        (status = 403, description = "未授權"),
        (status = 500, description = "內部伺服器錯誤")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn create_concert(
    State(state): State<AppState>,
    _admin_user: AdminUser,
    Json(input): Json<CreateConcert>,
) -> Result<Json<Concert>, AppError> {
    let concert = state.concert_service.create_concert(input, true).await?;
    Ok(Json(concert))
}

/// 獲取所有音樂會處理程序
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/concerts",
    responses(
        (status = 200, description = "成功獲取音樂會列表", body = Vec<Concert>)
    )
)]
pub async fn list_concerts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Concert>>, AppError> {
    let concerts = state.concert_service.get_all_concerts().await?;
    Ok(Json(concerts))
}
