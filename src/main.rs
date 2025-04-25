use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::docs::ApiDoc;
use crate::api::routes::create_router;
use crate::application::auth::service::AuthService;
use crate::application::concert::service::ConcertService;
use crate::application::order::service::OrderService;
use crate::application::ticket::service::TicketService;
use crate::config::AppConfig;
use crate::infrastructure::database::connection::init_pool;
use crate::infrastructure::database::repositories::concert_repository::PgConcertRepository;
use crate::infrastructure::database::repositories::order_repository::PgOrderRepository;
use crate::infrastructure::database::repositories::ticket_repository::PgTicketRepository;
use crate::infrastructure::database::repositories::user_repository::PgUserRepository;

mod api;
mod application;
mod config;
mod domain;
mod infrastructure;
mod utils;

#[tokio::main]
async fn main() {
    // 初始化配置
    let config = AppConfig::from_env();
    
    // 初始化日誌
    tracing_subscriber::fmt::init();
    
    // 初始化資料庫連接池
    let pool = init_pool().await.expect("無法連接到資料庫");
    
    // 初始化存儲庫
    let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
    let concert_repository = Arc::new(PgConcertRepository::new(pool.clone()));
    let ticket_repository = Arc::new(PgTicketRepository::new(pool.clone()));
    let order_repository = Arc::new(PgOrderRepository::new(pool.clone()));
    
    // 初始化服務
    let auth_service = Arc::new(AuthService::new(user_repository, config.jwt_secret.clone()));
    let concert_service = Arc::new(ConcertService::new(concert_repository.clone()));
    let ticket_service = Arc::new(TicketService::new(ticket_repository.clone(), concert_repository.clone()));
    let order_service = Arc::new(OrderService::new(order_repository, ticket_repository));
    
    // 創建 API 路由
    let app = create_router(
        auth_service,
        concert_service,
        ticket_service,
        order_service,
    )
    // 添加 Swagger UI
    .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
    
    // 添加中間件
    .layer(
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_origin(Any)
            .allow_headers(Any),
    )
    .layer(TraceLayer::new_for_http())
    .layer(RequestBodyLimitLayer::new(1024 * 1024)); // 1MB 限制
    
    // 啟動服務器
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("🚀 服務器運行在 http://{}/docs", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
