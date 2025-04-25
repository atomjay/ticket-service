use axum::{
    async_trait,
    extract::{FromRequestParts, State, Json},
    http::{request::Parts, StatusCode},
};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use utoipa::{ToSchema, IntoParams};
use validator::Validate;
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

// 建議放入 .env 再由 dotenvy 載入
pub const SECRET: &[u8] = b"super-secret-key";

/// JWT 內容結構
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // user_id
    pub exp: usize,    // 過期時間
    pub admin: bool,   // 是否為管理員
}

/// Extractor：解出 token 成為 AuthClaims
#[derive(Debug, Clone)]
pub struct AuthClaims(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = auth_header.trim_start_matches("Bearer ");

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthClaims(decoded.claims))
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

/// 用戶註冊
#[utoipa::path(
    post,
    path = "/api/users/register",
    request_body = RegisterInput,
    responses(
        (status = 201)
    )
)]
pub async fn register(
    State(pool): State<PgPool>,
    Json(input): Json<RegisterInput>,
) -> StatusCode {
    // 檢查郵箱是否已存在
    let exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists",
        input.email
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .exists
    .unwrap_or(false);

    if exists {
        return StatusCode::CONFLICT;
    }

    // 密碼雜湊
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(input.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // 創建用戶
    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash, is_admin) VALUES ($1, $2, $3)",
        input.email,
        password_hash,
        false
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Deserialize, ToSchema)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

/// 用戶登入
#[utoipa::path(
    post,
    path = "/api/users/login",
    request_body = LoginInput,
    responses(
        (status = 200, body = LoginResponse),
        (status = 401)
    )
)]
pub async fn login(
    State(pool): State<PgPool>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // 查詢用戶
    let user = sqlx::query!(
        "SELECT id, password_hash, is_admin FROM users WHERE email = $1",
        input.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user.ok_or(StatusCode::UNAUTHORIZED)?;

    // 驗證密碼
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let is_valid = Argon2::default()
        .verify_password(input.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 生成 JWT
    let now = Utc::now();
    let exp = (now + Duration::days(7)).timestamp() as usize;
    let claims = Claims {
        sub: user.id.to_string(),
        exp,
        admin: user.is_admin.unwrap_or(false),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token }))
}

#[derive(Serialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub is_admin: bool,
}

/// 獲取當前用戶信息
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, body = UserInfo)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_me(
    State(pool): State<PgPool>,
    AuthClaims(claims): AuthClaims,
) -> Result<Json<UserInfo>, StatusCode> {
    let user_id = claims.sub.parse::<Uuid>().unwrap();

    let u = sqlx::query!(
        "SELECT id, email, is_admin FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserInfo {
        id: u.id,
        email: u.email,
        is_admin: u.is_admin.unwrap_or(false),
    }))
}
