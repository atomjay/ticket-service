use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::order::model::{CreateOrder, Order, OrderQuery, OrderView};
use crate::utils::error::AppError;

/// 訂單存儲庫接口
#[async_trait]
pub trait OrderRepository: Send + Sync {
    /// 根據 ID 查找訂單
    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Option<OrderView>, AppError>;
    
    /// 根據用戶 ID 查找訂單
    async fn find_by_user_id(&self, user_id: Uuid, query: &OrderQuery) -> Result<Vec<OrderView>, AppError>;
    
    /// 創建新訂單
    async fn create(&self, user_id: Uuid, input: &CreateOrder) -> Result<Order, AppError>;
}
