use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, header::AUTHORIZATION},
    response::{IntoResponse, Response},
};

use crate::application::auth::service::AuthService;
use crate::domain::auth::model::User;
use crate::utils::error::AppError;
use crate::api::routes::AppState;

/// 認證中間件
pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    /// 創建新的認證中間件
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }
}

/// 用戶認證提取器
pub struct AuthUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 從請求中提取 Authorization 頭
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("未提供認證令牌".to_string()).into_response())?;

        // 檢查 Bearer 前綴
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("無效的認證令牌格式".to_string()).into_response());
        }

        let token = auth_header.trim_start_matches("Bearer ");

        // 從請求中提取 AppState
        let app_state = match parts.extensions.get::<AppState>() {
            Some(state) => state,
            None => return Err(AppError::Internal("無法獲取應用程式狀態".to_string()).into_response()),
        };

        // 從 AppState 中獲取認證服務
        let auth_service = &app_state.auth_service;

        // 驗證令牌並獲取用戶
        let user = auth_service.get_user_from_token(token)
            .await
            .map_err(|e| e.into_response())?;

        Ok(Self(user))
    }
}

/// 管理員認證提取器
pub struct AdminUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 先驗證用戶認證
        match AuthUser::from_request_parts(parts, state).await {
            Ok(AuthUser(user)) => {
                // 檢查是否為管理員
                if !user.is_admin {
                    Err(AppError::Forbidden("需要管理員權限".to_string()).into_response())
                } else {
                    Ok(Self(user))
                }
            },
            Err(e) => Err(e),
        }
    }
}
