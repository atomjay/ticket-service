use std::sync::Arc;
use uuid::Uuid;

use crate::domain::concert::repository::ConcertRepository;
use crate::domain::ticket::model::{CreateTicket, Ticket};
use crate::domain::ticket::repository::TicketRepository;
use crate::utils::error::AppError;

/// 票券服務
pub struct TicketService {
    ticket_repository: Arc<dyn TicketRepository>,
    concert_repository: Arc<dyn ConcertRepository>,
}

impl TicketService {
    /// 創建新的票券服務實例
    pub fn new(
        ticket_repository: Arc<dyn TicketRepository>,
        concert_repository: Arc<dyn ConcertRepository>,
    ) -> Self {
        Self {
            ticket_repository,
            concert_repository,
        }
    }

    /// 創建新票券
    pub async fn create_ticket(&self, input: CreateTicket, is_admin: bool) -> Result<Ticket, AppError> {
        // 檢查權限
        if !is_admin {
            return Err(AppError::Forbidden("需要管理員權限".to_string()));
        }

        // 檢查音樂會是否存在
        self.concert_repository.find_by_id(input.concert_id).await?
            .ok_or_else(|| AppError::NotFound(format!("找不到 ID 為 {} 的音樂會", input.concert_id)))?;

        // 創建票券
        self.ticket_repository.create(&input).await
    }

    /// 根據音樂會 ID 獲取票券
    pub async fn get_tickets_by_concert_id(&self, concert_id: Uuid) -> Result<Vec<Ticket>, AppError> {
        self.ticket_repository.find_by_concert_id(concert_id).await
    }

    /// 根據 ID 獲取票券
    pub async fn get_ticket_by_id(&self, id: Uuid) -> Result<Ticket, AppError> {
        self.ticket_repository.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("找不到 ID 為 {} 的票券", id)))
    }
}
