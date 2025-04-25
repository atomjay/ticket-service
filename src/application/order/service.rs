use std::sync::Arc;
use uuid::Uuid;

use crate::domain::order::model::{CreateOrder, Order, OrderQuery, OrderView};
use crate::domain::order::repository::OrderRepository;
use crate::domain::ticket::repository::TicketRepository;
use crate::utils::error::AppError;

/// 訂單服務
pub struct OrderService {
    order_repository: Arc<dyn OrderRepository>,
    ticket_repository: Arc<dyn TicketRepository>,
}

impl OrderService {
    /// 創建新的訂單服務實例
    pub fn new(
        order_repository: Arc<dyn OrderRepository>,
        ticket_repository: Arc<dyn TicketRepository>,
    ) -> Self {
        Self {
            order_repository,
            ticket_repository,
        }
    }

    /// 創建新訂單
    pub async fn create_order(&self, user_id: Uuid, input: CreateOrder) -> Result<Order, AppError> {
        // 開始資料庫事務
        // 注意：實際實現會在存儲庫層處理事務

        // 檢查票券是否存在並獲取庫存
        let ticket = self.ticket_repository.find_by_id(input.ticket_id).await?
            .ok_or_else(|| AppError::NotFound(format!("找不到 ID 為 {} 的票券", input.ticket_id)))?;

        // 檢查庫存是否足夠
        if ticket.stock < input.quantity {
            return Err(AppError::BadRequest("庫存不足".to_string()));
        }

        // 更新票券庫存
        self.ticket_repository.update_stock(input.ticket_id, input.quantity).await?;

        // 創建訂單
        let order = self.order_repository.create(user_id, &input).await?;

        // 提交事務
        Ok(order)
    }

    /// 獲取用戶訂單列表
    pub async fn get_user_orders(&self, user_id: Uuid, query: OrderQuery) -> Result<Vec<OrderView>, AppError> {
        self.order_repository.find_by_user_id(user_id, &query).await
    }

    /// 根據 ID 獲取訂單
    pub async fn get_order_by_id(&self, id: Uuid, user_id: Uuid) -> Result<OrderView, AppError> {
        self.order_repository.find_by_id(id, user_id).await?
            .ok_or_else(|| AppError::NotFound(format!("找不到 ID 為 {} 的訂單", id)))
    }
}
