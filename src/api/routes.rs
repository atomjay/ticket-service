use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::api::handlers::{
    auth_handler::{get_me, login, register},
    concert_handler::{create_concert, list_concerts},
    order_handler::{create_order, get_order_by_id, list_orders},
    ticket_handler::{create_ticket, list_tickets},
};
use crate::application::auth::service::AuthService;
use crate::application::concert::service::ConcertService;
use crate::application::order::service::OrderService;
use crate::application::ticket::service::TicketService;

// 定義應用程式狀態類型
#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub concert_service: Arc<ConcertService>,
    pub ticket_service: Arc<TicketService>,
    pub order_service: Arc<OrderService>,
}

/// 創建 API 路由
pub fn create_router(
    auth_service: Arc<AuthService>,
    concert_service: Arc<ConcertService>,
    ticket_service: Arc<TicketService>,
    order_service: Arc<OrderService>,
) -> Router {
    // 創建共享狀態
    let state = AppState {
        auth_service,
        concert_service,
        ticket_service,
        order_service,
    };
    
    Router::new()
        // === 用戶認證 ===
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(get_me))
        // === 音樂會 ===
        .route("/concerts", 
            get(list_concerts)
            .post(create_concert)
        )
        // === 票券 ===
        .route("/tickets", 
            get(list_tickets)
            .post(create_ticket)
        )
        // === 訂單 ===
        .route("/orders", 
            get(list_orders)
            .post(create_order)
        )
        .route("/orders/:order_id", get(get_order_by_id))
        // === 狀態 ===
        .with_state(state)
}
