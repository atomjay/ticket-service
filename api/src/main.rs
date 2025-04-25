use axum::{
    routing::{get, post},
    Router, extract::{State, Json},
    http::StatusCode,
};
use corelib::{
    db::init_pool,
    auth::{register, login, get_me, AuthClaims},
    concert::create_concert,
    ticket::{CreateTicket, list_tickets},
    order::{CreateOrder, list_orders, get_order_by_id},
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tower_http::{
    trace::TraceLayer,
    limit::RequestBodyLimitLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use sqlx::PgPool;

// 導入 docs 模組和 ApiDoc
mod docs;
use docs::ApiDoc;

// 使用 debug_handler 宏來解決 Handler trait 問題
#[axum::debug_handler]
async fn create_ticket_handler(
    state: State<PgPool>,
    claims: AuthClaims,
    json: Json<CreateTicket>,
) -> StatusCode {
    corelib::ticket::create_ticket(state, claims, json).await
}

#[axum::debug_handler]
async fn create_order_handler(
    state: State<PgPool>,
    claims: AuthClaims,
    json: Json<CreateOrder>,
) -> StatusCode {
    corelib::order::create_order(state, claims, json).await
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // 載入 .env
    tracing_subscriber::fmt::init();

    let pool = init_pool().await.expect("Failed to connect to DB");

    let app = Router::new()
        // === User 認證 ===
        .route("/api/users/register", post(register))
        .route("/api/users/login", post(login))
        .route("/api/users/me", get(get_me))

        // === 演唱會與票券 ===
        .route("/api/concerts", post(create_concert))
        .route("/api/tickets", post(create_ticket_handler).get(list_tickets))

        // === 訂單系統 ===
        .route("/api/orders", post(create_order_handler).get(list_orders))
        .route("/api/orders/:id", get(get_order_by_id))

        // === Swagger 文件 ===
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))

        // === 狀態與共享連線 ===
        .with_state(pool)

        // === Middleware ===
        .layer(TraceLayer::new_for_http()) // logging
        .layer(RequestBodyLimitLayer::new(1024 * 10)); // 限制請求體大小

    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));

    println!("🚀 Server running at http://{}/docs", addr);
    
    // 修正 Server 引用
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
