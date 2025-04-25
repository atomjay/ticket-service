// 引入標準庫中的線程安全引用計數類型
use std::sync::Arc;

// 引入 Axum 框架的路由相關功能
use axum::{
    // 引入 HTTP 請求方法路由功能
    // get 用於處理 GET 請求（獲取資源）
    // post 用於處理 POST 請求（創建資源）
    routing::{get, post},
    // Router 是 Axum 的核心組件，用於定義 API 路由
    Router,
};

// 引入我們自己定義的處理器函數
use crate::api::handlers::{
    // 認證相關處理器
    auth_handler::{get_me, login, register},
    // 演唱會相關處理器
    concert_handler::{create_concert, list_concerts},
    // 訂單相關處理器
    order_handler::{create_order, get_order_by_id, list_orders},
    // 票券相關處理器
    ticket_handler::{create_ticket, list_tickets},
};
// 引入應用服務
use crate::application::auth::service::AuthService;
use crate::application::concert::service::ConcertService;
use crate::application::order::service::OrderService;
use crate::application::ticket::service::TicketService;

// 定義應用程式狀態類型
// 這個結構體包含了所有服務的引用，將被傳遞給每個處理器
// #[derive(Clone)] 自動為結構體實現 Clone 特性，允許我們創建結構體的副本
#[derive(Clone)]
pub struct AppState {
    // 認證服務，處理用戶登錄、註冊等功能
    pub auth_service: Arc<AuthService>,
    // 演唱會服務，處理演唱會相關邏輯
    pub concert_service: Arc<ConcertService>,
    // 票券服務，處理票券相關邏輯
    pub ticket_service: Arc<TicketService>,
    // 訂單服務，處理訂單相關邏輯
    pub order_service: Arc<OrderService>,
}

/// 創建 API 路由
/// 這個函數定義了所有 API 端點及其對應的處理器函數
/// 
/// # 參數
/// * `auth_service` - 認證服務的引用
/// * `concert_service` - 演唱會服務的引用
/// * `ticket_service` - 票券服務的引用
/// * `order_service` - 訂單服務的引用
/// 
/// # 返回值
/// 返回配置好的 Axum Router 實例，包含所有 API 路由
pub fn create_router(
    auth_service: Arc<AuthService>,
    concert_service: Arc<ConcertService>,
    ticket_service: Arc<TicketService>,
    order_service: Arc<OrderService>,
) -> Router {
    // 創建共享狀態
    // 這個狀態將被傳遞給所有處理器函數
    let state = AppState {
        auth_service,
        concert_service,
        ticket_service,
        order_service,
    };
    
    // 創建新的路由器並定義所有 API 端點
    Router::new()
        // === 用戶認證 API ===
        // 用戶註冊端點：接收 POST 請求，創建新用戶
        .route("/auth/register", post(register))
        // 用戶登錄端點：接收 POST 請求，驗證用戶憑證並返回 JWT 令牌
        .route("/auth/login", post(login))
        // 獲取當前用戶信息端點：接收 GET 請求，返回已認證用戶的詳細信息
        .route("/auth/me", get(get_me))
        
        // === 音樂會 API ===
        // 演唱會端點：
        // - GET 請求獲取所有演唱會列表
        // - POST 請求創建新演唱會（需要管理員權限）
        .route("/concerts", 
            get(list_concerts)
            .post(create_concert)
        )
        
        // === 票券 API ===
        // 票券端點：
        // - GET 請求獲取所有票券列表
        // - POST 請求創建新票券（需要管理員權限）
        .route("/tickets", 
            get(list_tickets)
            .post(create_ticket)
        )
        
        // === 訂單 API ===
        // 訂單端點：
        // - GET 請求獲取當前用戶的所有訂單
        // - POST 請求創建新訂單（購買票券）
        .route("/orders", 
            get(list_orders)
            .post(create_order)
        )
        // 獲取特定訂單詳情端點：接收 GET 請求，返回指定 ID 的訂單詳情
        // :order_id 是路徑參數，表示訂單的唯一標識符
        .route("/orders/:order_id", get(get_order_by_id))
        
        // === 添加應用狀態 ===
        // 將創建的狀態附加到路由器
        // 這樣所有處理器函數都可以訪問這些服務
        .with_state(state)
}
