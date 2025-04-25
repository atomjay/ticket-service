use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::domain::auth::model::{Claims, LoginInput, LoginResponse, RegisterInput, User};
use crate::domain::auth::repository::UserRepository;
use crate::infrastructure::security::password;
use crate::utils::error::AppError;

/// 認證服務
pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    jwt_secret: String,
}

impl AuthService {
    /// 創建新的認證服務實例
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    /// 註冊新用戶
    pub async fn register(&self, input: RegisterInput) -> Result<(), AppError> {
        // 檢查電子郵件是否已存在
        if self.user_repository.email_exists(&input.email).await? {
            return Err(AppError::Conflict("電子郵件已存在".to_string()));
        }

        // 雜湊密碼
        let password_hash = password::hash_password(&input.password)?;

        // 創建用戶
        self.user_repository.create(&input, &password_hash).await?;

        Ok(())
    }

    /// 用戶登入
    pub async fn login(&self, input: LoginInput) -> Result<LoginResponse, AppError> {
        // 查找用戶
        let user = self.user_repository.find_by_email(&input.email).await?
            .ok_or_else(|| AppError::Unauthorized("無效的憑證".to_string()))?;

        // 驗證密碼
        if !password::verify_password(&input.password, &user.password_hash)? {
            return Err(AppError::Unauthorized("無效的憑證".to_string()));
        }

        // 生成 JWT
        let token = self.generate_token(&user)?;

        Ok(LoginResponse {
            token,
            is_admin: user.is_admin,
        })
    }

    /// 驗證 JWT 並獲取用戶
    pub async fn get_user_from_token(&self, token: &str) -> Result<User, AppError> {
        // 解析 JWT 獲取用戶 ID
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("無效的令牌".to_string()))?
        .claims;

        // 查找用戶
        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("無效的令牌".to_string()))?;

        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| AppError::Unauthorized("用戶不存在".to_string()))?;

        Ok(user)
    }

    /// 生成 JWT
    fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("有效的時間戳")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            admin: user.is_admin,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal("無法生成令牌".to_string()))
    }
}
