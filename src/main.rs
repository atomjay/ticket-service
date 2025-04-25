// 引入標準庫中的網絡地址和線程安全引用計數類型
use std::net::SocketAddr;
use std::sync::Arc;  // Arc 是 Atomic Reference Counting 的縮寫，用於在多線程環境中安全地共享數據

// 引入 Axum 框架的 HTTP 方法類型
use axum::http::Method;
// 引入跨域資源共享(CORS)相關功能，允許不同網站的前端訪問我們的 API
use tower_http::cors::{Any, CorsLayer};
// 引入請求體大小限制功能，防止過大的請求導致服務器負擔過重
use tower_http::limit::RequestBodyLimitLayer;
// 引入請求追蹤功能，用於記錄和監控 API 請求
use tower_http::trace::TraceLayer;
// 引入 OpenAPI 文檔生成工具，用於自動生成 API 文檔
use utoipa::OpenApi;
// 引入 Swagger UI 工具，提供一個網頁界面來查看和測試 API
use utoipa_swagger_ui::SwaggerUi;

// 引入我們自己定義的模塊和類型
// API 文檔定義
use crate::api::docs::ApiDoc;
// 路由創建函數
use crate::api::routes::create_router;
// 認證服務，處理用戶登錄、註冊等功能
use crate::application::auth::service::AuthService;
// 演唱會服務，處理演唱會相關邏輯
use crate::application::concert::service::ConcertService;
// 訂單服務，處理訂單相關邏輯
use crate::application::order::service::OrderService;
// 票券服務，處理票券相關邏輯
use crate::application::ticket::service::TicketService;
// 應用程序配置，從環境變量中讀取配置信息
use crate::config::AppConfig;
// 數據庫連接池初始化函數
use crate::infrastructure::database::connection::init_pool;
// 各種資料庫存儲庫的實現
use crate::infrastructure::database::repositories::concert_repository::PgConcertRepository;
use crate::infrastructure::database::repositories::order_repository::PgOrderRepository;
use crate::infrastructure::database::repositories::ticket_repository::PgTicketRepository;
use crate::infrastructure::database::repositories::user_repository::PgUserRepository;

// 聲明我們的模塊結構
// API 層：處理 HTTP 請求和響應
mod api;
// 應用層：實現業務邏輯
mod application;
// 配置層：處理應用程序配置
mod config;
// 領域層：定義核心業務模型和規則
mod domain;
// 基礎設施層：提供技術實現，如數據庫訪問
mod infrastructure;
// 工具層：提供通用工具函數
mod utils;

// 標記這是一個 Tokio 異步運行時的主函數
// 這是應用程序的入口點，所有執行從這裡開始
#[tokio::main]
async fn main() {
    // 從環境變量中讀取應用程序配置
    // 包括數據庫連接字符串、JWT 密鑰、服務器端口等
    let config = AppConfig::from_env();
    
    // 初始化日誌系統，用於記錄應用程序運行時的信息
    // 這對於調試和監控應用程序非常重要
    tracing_subscriber::fmt::init();
    
    // 初始化數據庫連接池
    // 連接池允許應用程序重用數據庫連接，提高性能
    // expect 是錯誤處理：如果連接失敗，程序會終止並顯示錯誤信息
    let pool = init_pool().await.expect("無法連接到資料庫");
    
    // 初始化各種存儲庫
    // 存儲庫負責與數據庫交互，執行 CRUD 操作
    // Arc 使這些存儲庫可以在多個服務之間安全共享
    // pool.clone() 是複製的是資料庫連接池的智慧指針（增加引用計數），而非實際建立新連線，避免重複建立連線造成的資源浪費
    let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
    let concert_repository = Arc::new(PgConcertRepository::new(pool.clone()));
    let ticket_repository = Arc::new(PgTicketRepository::new(pool.clone()));
    let order_repository = Arc::new(PgOrderRepository::new(pool.clone()));
    
    // 初始化各種服務
    // 服務實現業務邏輯，使用存儲庫來訪問數據
    let auth_service = Arc::new(AuthService::new(user_repository, config.jwt_secret.clone()));
    let concert_service = Arc::new(ConcertService::new(concert_repository.clone()));
    let ticket_service = Arc::new(TicketService::new(ticket_repository.clone(), concert_repository.clone()));
    let order_service = Arc::new(OrderService::new(order_repository, ticket_repository));
    
    // 創建 API 路由
    // 路由定義了 HTTP 請求如何映射到處理函數
    let app = create_router(
        auth_service,
        concert_service,
        ticket_service,
        order_service,
    )
    // 添加 Swagger UI
    // 這提供了一個網頁界面，可以查看和測試 API
    .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
    
    // 添加中間件
    // 中間件是在請求處理前後執行的功能
    .layer(
        // CORS 中間件允許不同網站的前端訪問我們的 API
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_origin(Any)  // 允許任何來源的請求
            .allow_headers(Any),  // 允許任何 HTTP 頭部
    )
    // 追蹤中間件記錄請求和響應信息，有助於調試
    .layer(TraceLayer::new_for_http())
    // 限制請求體大小為 1MB，防止過大的請求
    .layer(RequestBodyLimitLayer::new(1024 * 1024)); // 1MB 限制
    
    // 設置服務器地址和端口
    // 127.0.0.1 是本地地址，只能從同一台機器訪問
    // config.port 是從配置中讀取的端口號
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("🚀 服務器運行在 http://{}/docs", addr);
    
    // 創建 TCP 監聽器並啟動服務器
    // 監聽器等待客戶端連接
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // 啟動 Axum 服務器，處理傳入的 HTTP 請求
    // unwrap() 在出錯時會導致程序崩潰，這裡用於簡化錯誤處理
    axum::serve(listener, app).await.unwrap();
}
