use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::api::middleware::auth::AuthUser;
use crate::api::routes::AppState;
use crate::domain::order::model::{CreateOrder, OrderQuery, OrderView};
use crate::utils::error::AppError;

/// 創建訂單處理程序
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/orders",
    request_body = CreateOrder,
    responses(
        (status = 201, description = "訂單創建成功"),
        (status = 400, description = "無效的輸入數據"),
        (status = 401, description = "未認證"),
        (status = 404, description = "票券不存在"),
        (status = 500, description = "內部伺服器錯誤")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn create_order(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(input): Json<CreateOrder>,
) -> Result<StatusCode, AppError> {
    state.order_service.create_order(auth_user.0.id, input).await?;
    Ok(StatusCode::CREATED)
}

/// 獲取用戶訂單列表處理程序
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/orders",
    params(
        OrderQuery
    ),
    responses(
        (status = 200, description = "成功獲取訂單列表", body = Vec<OrderView>),
        (status = 401, description = "未授權訪問")
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "orders"
)]
pub async fn list_orders(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<OrderQuery>,
) -> Result<Json<Vec<OrderView>>, AppError> {
    let orders = state.order_service.get_user_orders(auth_user.0.id, query).await?;
    Ok(Json(orders))
}

/// 獲取訂單詳情處理程序
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/orders/{order_id}",
    params(
        ("order_id" = Uuid, Path, description = "訂單 ID")
    ),
    responses(
        (status = 200, description = "成功獲取訂單詳情", body = OrderView),
        (status = 401, description = "未授權訪問"),
        (status = 404, description = "訂單不存在")
    ),
    security(
        ("jwt_auth" = [])
    ),
    tag = "orders"
)]
pub async fn get_order_by_id(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderView>, AppError> {
    let order = state.order_service.get_order_by_id(order_id, auth_user.0.id).await?;
    Ok(Json(order))
}
