use std::sync::Arc;

use crate::domain::concert::model::{Concert, CreateConcert};
use crate::domain::concert::repository::ConcertRepository;
use crate::utils::error::AppError;

/// 演唱會服務
pub struct ConcertService {
    concert_repository: Arc<dyn ConcertRepository>,
}

impl ConcertService {
    /// 創建新的演唱會服務實例
    pub fn new(concert_repository: Arc<dyn ConcertRepository>) -> Self {
        Self {
            concert_repository,
        }
    }

    /// 創建新演唱會
    pub async fn create_concert(&self, input: CreateConcert, is_admin: bool) -> Result<Concert, AppError> {
        // 檢查權限
        if !is_admin {
            return Err(AppError::Forbidden("需要管理員權限".to_string()));
        }

        // 創建演唱會
        self.concert_repository.create(&input).await
    }

    /// 獲取所有演唱會
    pub async fn get_all_concerts(&self) -> Result<Vec<Concert>, AppError> {
        self.concert_repository.find_all().await
    }

    /// 根據 ID 獲取演唱會
    pub async fn get_concert_by_id(&self, id: uuid::Uuid) -> Result<Concert, AppError> {
        self.concert_repository.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("找不到 ID 為 {} 的演唱會", id)))
    }
}
