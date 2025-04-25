use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// 應用程式錯誤類型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("認證錯誤: {0}")]
    Unauthorized(String),

    #[error("權限不足: {0}")]
    Forbidden(String),

    #[error("資源不存在: {0}")]
    NotFound(String),

    #[error("請求無效: {0}")]
    BadRequest(String),

    #[error("資源衝突: {0}")]
    Conflict(String),

    #[error("資料庫錯誤: {0}")]
    Database(#[from] sqlx::Error),

    #[error("內部伺服器錯誤: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Conflict(message) => (StatusCode::CONFLICT, message),
            AppError::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("資料庫錯誤: {}", err),
            ),
            AppError::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
