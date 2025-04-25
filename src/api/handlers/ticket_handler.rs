use axum::{
    extract::{Json, Query, State},
};

use crate::api::middleware::auth::AdminUser;
use crate::api::routes::AppState;
use crate::domain::ticket::model::{CreateTicket, Ticket, TicketQuery};
use crate::utils::error::AppError;

/// 創建票券處理程序
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/tickets",
    request_body = CreateTicket,
    responses(
        (status = 200, description = "票券創建成功", body = Ticket),
        (status = 400, description = "無效的輸入數據"),
        (status = 401, description = "未認證"),
        (status = 403, description = "未授權"),
        (status = 404, description = "音樂會不存在"),
        (status = 500, description = "內部伺服器錯誤")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
pub async fn create_ticket(
    State(state): State<AppState>,
    _admin_user: AdminUser,
    Json(input): Json<CreateTicket>,
) -> Result<Json<Ticket>, AppError> {
    let ticket = state.ticket_service.create_ticket(input, true).await?;
    Ok(Json(ticket))
}

/// 獲取票券列表處理程序
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/tickets",
    params(
        TicketQuery
    ),
    responses(
        (status = 200, description = "成功獲取票券列表", body = Vec<Ticket>),
        (status = 404, description = "演唱會不存在")
    ),
    tag = "tickets"
)]
pub async fn list_tickets(
    State(state): State<AppState>,
    Query(query): Query<TicketQuery>,
) -> Result<Json<Vec<Ticket>>, AppError> {
    let tickets = state.ticket_service.get_tickets_by_concert_id(query.concert_id).await?;
    Ok(Json(tickets))
}
