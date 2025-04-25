use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::concert::model::{Concert, CreateConcert};
use crate::utils::error::AppError;

/// 演唱會存儲庫接口
#[async_trait]
pub trait ConcertRepository: Send + Sync {
    /// 根據 ID 查找演唱會
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Concert>, AppError>;
    
    /// 獲取所有演唱會
    async fn find_all(&self) -> Result<Vec<Concert>, AppError>;
    
    /// 創建新演唱會
    async fn create(&self, input: &CreateConcert) -> Result<Concert, AppError>;
}
