use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

/// 使用者模型
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
}

/// 使用者註冊輸入
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
}

/// 使用者登入輸入
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

/// 登入響應
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub is_admin: bool,
}

/// JWT 聲明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub admin: bool,
    pub exp: usize,
}
