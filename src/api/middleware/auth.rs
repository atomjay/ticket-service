// 引入標準庫中的線程安全引用計數類型
use std::sync::Arc;

// 引入 Axum 框架的相關功能
use axum::{
    // async_trait 是一個宏，用於在 trait 中使用異步函數
    async_trait,
    // FromRequestParts 是一個 trait，允許從 HTTP 請求中提取數據
    extract::{FromRequestParts},
    // 引入 HTTP 相關類型，用於處理請求和響應
    http::{request::Parts, header::AUTHORIZATION},
    // 用於將自定義類型轉換為 HTTP 響應
    response::{IntoResponse, Response},
};

// 引入我們自己定義的模塊和類型
// 認證服務，處理令牌驗證等功能
use crate::application::auth::service::AuthService;
// 用戶模型，包含用戶信息
use crate::domain::auth::model::User;
// 應用程序錯誤類型，用於統一錯誤處理
use crate::utils::error::AppError;
// 應用程序狀態，包含所有服務的引用
use crate::api::routes::AppState;

/// 認證中間件
/// 這個結構體封裝了認證服務，用於處理用戶身份驗證
pub struct AuthMiddleware {
    // 認證服務的引用，使用 Arc 實現線程安全的共享
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    /// 創建新的認證中間件
    /// 
    /// # 參數
    /// * `auth_service` - 認證服務的引用
    /// 
    /// # 返回值
    /// 返回一個新的 AuthMiddleware 實例
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }
}

/// 用戶認證提取器
/// 這個結構體用於從 HTTP 請求中提取已認證的用戶
/// 它包裝了一個 User 實例，表示當前認證的用戶
pub struct AuthUser(pub User);

// async_trait 宏允許在 trait 中使用異步函數
#[async_trait]
// 實現 FromRequestParts trait，使 AuthUser 可以從 HTTP 請求中提取
// 泛型參數 S 表示狀態類型，通常是 AppState
// where 子句限制 S 必須是可以安全地在線程間發送和共享的類型
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    // 如果提取失敗，返回的錯誤類型
    type Rejection = Response;

    /// 從 HTTP 請求部分中提取用戶
    /// 
    /// # 參數
    /// * `parts` - HTTP 請求的各個部分
    /// * `_state` - 應用程序狀態（這裡未使用，所以前綴為 _）
    /// 
    /// # 返回值
    /// 如果成功，返回包含用戶的 AuthUser
    /// 如果失敗，返回錯誤響應
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 從請求中提取 Authorization 頭
        // 1. 獲取 Authorization 頭部
        // 2. 將頭部值轉換為字符串
        // 3. 如果任何步驟失敗，返回未授權錯誤
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("未提供認證令牌".to_string()).into_response())?;

        // 檢查 Bearer 前綴
        // JWT 令牌通常以 "Bearer " 開頭
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("無效的認證令牌格式".to_string()).into_response());
        }

        // 提取實際的令牌，去除 "Bearer " 前綴
        let token = auth_header.trim_start_matches("Bearer ");

        // 從請求中提取 AppState
        // 使用 extensions 獲取之前設置的應用程序狀態
        let app_state = match parts.extensions.get::<AppState>() {
            Some(state) => state,
            None => return Err(AppError::Internal("無法獲取應用程式狀態".to_string()).into_response()),
        };

        // 從 AppState 中獲取認證服務
        let auth_service = &app_state.auth_service;

        // 驗證令牌並獲取用戶
        // 調用認證服務的方法來驗證令牌並獲取對應的用戶
        // 如果失敗，將錯誤轉換為 HTTP 響應
        let user = auth_service.get_user_from_token(token)
            .await
            .map_err(|e| e.into_response())?;

        // 返回包含用戶的 AuthUser 實例
        Ok(Self(user))
    }
}

/// 管理員認證提取器
/// 這個結構體用於從 HTTP 請求中提取已認證的管理員用戶
/// 它包裝了一個 User 實例，表示當前認證的管理員用戶
pub struct AdminUser(pub User);

// async_trait 宏允許在 trait 中使用異步函數
#[async_trait]
// 實現 FromRequestParts trait，使 AdminUser 可以從 HTTP 請求中提取
impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
{
    // 如果提取失敗，返回的錯誤類型
    type Rejection = Response;

    /// 從 HTTP 請求部分中提取管理員用戶
    /// 
    /// # 參數
    /// * `parts` - HTTP 請求的各個部分
    /// * `state` - 應用程序狀態
    /// 
    /// # 返回值
    /// 如果成功，返回包含管理員用戶的 AdminUser
    /// 如果失敗，返回錯誤響應
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 先驗證用戶認證
        // 使用 AuthUser 提取器獲取已認證的用戶
        match AuthUser::from_request_parts(parts, state).await {
            Ok(AuthUser(user)) => {
                // 檢查是否為管理員
                // 如果用戶不是管理員，返回禁止訪問錯誤
                if !user.is_admin {
                    Err(AppError::Forbidden("需要管理員權限".to_string()).into_response())
                } else {
                    // 如果是管理員，返回包含用戶的 AdminUser 實例
                    Ok(Self(user))
                }
            },
            // 如果用戶認證失敗，直接返回錯誤
            Err(e) => Err(e),
        }
    }
}
