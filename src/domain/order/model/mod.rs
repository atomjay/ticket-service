use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// 訂單模型
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ticket_id: Uuid,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
}

/// 創建訂單輸入
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateOrder {
    pub ticket_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

/// 訂單視圖（包含關聯資訊）
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct OrderView {
    pub id: Uuid,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub ticket_type: String,
    pub price: f64,
    pub concert_title: String,
    pub concert_date: NaiveDateTime,
}

/// 訂單查詢參數
#[derive(Debug, Clone, Deserialize, IntoParams, ToSchema)]
pub struct OrderQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub concert_id: Option<Uuid>,
}
