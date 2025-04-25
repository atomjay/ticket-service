use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{ToSchema, IntoParams};
use validator::Validate;

/// 票券模型
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Ticket {
    pub id: Uuid,
    pub concert_id: Uuid,
    pub ticket_type: String,
    pub price: f64,
    pub stock: i32,
}

/// 創建票券輸入
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateTicket {
    pub concert_id: Uuid,
    pub ticket_type: String,
    pub price: f64,
    pub stock: i32,
}

/// 票券查詢參數
#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct TicketQuery {
    pub concert_id: Uuid,
}
