use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::auth::model::{RegisterInput, User};
use crate::utils::error::AppError;

/// 用戶存儲庫介面
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 創建新用戶
    async fn create(&self, input: &RegisterInput, password_hash: &str) -> Result<(), AppError>;

    /// 根據電子郵件查找用戶
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    /// 根據 ID 查找用戶
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;

    /// 檢查電子郵件是否已存在
    async fn email_exists(&self, email: &str) -> Result<bool, AppError>;
}
