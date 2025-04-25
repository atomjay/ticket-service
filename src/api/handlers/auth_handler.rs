use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::api::middleware::auth::AuthUser;
use crate::api::routes::AppState;
use crate::domain::auth::model::{LoginInput, LoginResponse, RegisterInput};
use crate::utils::error::AppError;

/// 用戶註冊處理程序
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/users/register",
    request_body = RegisterInput,
    responses(
        (status = 201, description = "用戶註冊成功"),
        (status = 400, description = "無效的輸入數據"),
        (status = 409, description = "電子郵件已被使用")
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterInput>,
) -> Result<StatusCode, AppError> {
    state.auth_service.register(input).await?;
    Ok(StatusCode::CREATED)
}

/// 用戶登入處理程序
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/users/login",
    request_body = LoginInput,
    responses(
        (status = 200, description = "登入成功", body = LoginResponse),
        (status = 400, description = "無效的登入憑證")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = state.auth_service.login(input).await?;
    Ok(Json(response))
}

/// 獲取當前用戶信息處理程序
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "成功獲取用戶信息"),
        (status = 401, description = "未授權訪問")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "auth"
)]
pub async fn get_me(
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = auth_user.0;
    
    Ok(Json(serde_json::json!({
        "id": user.id,
        "email": user.email,
        "is_admin": user.is_admin
    })))
}
