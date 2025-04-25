use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

/// 演唱會模型
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Concert {
    pub id: Uuid,
    pub title: String,
    pub artist: String,
    pub venue: String,
    pub date: NaiveDateTime,
}

/// 創建演唱會輸入
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateConcert {
    pub title: String,
    pub artist: String,
    pub venue: String,
    pub date: NaiveDateTime,
}
