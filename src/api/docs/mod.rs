use utoipa::OpenApi;

use crate::domain::auth::model::{LoginInput, LoginResponse, RegisterInput};
use crate::domain::concert::model::{Concert, CreateConcert};
use crate::domain::order::model::{CreateOrder, OrderQuery, OrderView};
use crate::domain::ticket::model::{CreateTicket, Ticket, TicketQuery};

/// API 文檔
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::auth_handler::register,
        crate::api::handlers::auth_handler::login,
        crate::api::handlers::auth_handler::get_me,
        crate::api::handlers::concert_handler::create_concert,
        crate::api::handlers::concert_handler::list_concerts,
        crate::api::handlers::ticket_handler::create_ticket,
        crate::api::handlers::ticket_handler::list_tickets,
        crate::api::handlers::order_handler::create_order,
        crate::api::handlers::order_handler::list_orders,
        crate::api::handlers::order_handler::get_order_by_id,
    ),
    components(
        schemas(
            RegisterInput,
            LoginInput,
            LoginResponse,
            Concert,
            CreateConcert,
            Ticket,
            CreateTicket,
            TicketQuery,
            OrderView,
            CreateOrder,
            OrderQuery,
        )
    ),
    tags(
        (name = "auth", description = "用戶認證 API"),
        (name = "concerts", description = "演唱會 API"),
        (name = "tickets", description = "票券 API"),
        (name = "orders", description = "訂單 API"),
    ),
    info(
        title = "票務系統 API",
        version = "1.0.0",
        description = "票務系統 RESTful API 文檔",
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub struct ApiDoc;
