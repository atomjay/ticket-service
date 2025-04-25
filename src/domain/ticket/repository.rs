use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::ticket::model::{CreateTicket, Ticket};
use crate::utils::error::AppError;

/// 票券存儲庫接口
#[async_trait]
pub trait TicketRepository: Send + Sync {
    /// 根據 ID 查找票券
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Ticket>, AppError>;
    
    /// 根據演唱會 ID 查找票券
    async fn find_by_concert_id(&self, concert_id: Uuid) -> Result<Vec<Ticket>, AppError>;
    
    /// 創建新票券
    async fn create(&self, input: &CreateTicket) -> Result<Ticket, AppError>;
    
    /// 更新票券庫存
    async fn update_stock(&self, id: Uuid, quantity: i32) -> Result<(), AppError>;
}
