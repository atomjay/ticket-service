use corelib::auth::{RegisterInput, LoginInput, LoginResponse, UserInfo};
use corelib::ticket::{CreateTicket, Ticket};
use corelib::order::{CreateOrder, OrderQuery, OrderView};
use corelib::concert::CreateConcert;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        corelib::auth::register,
        corelib::auth::login,
        corelib::auth::get_me,
        corelib::concert::create_concert,
        corelib::ticket::create_ticket,
        corelib::ticket::list_tickets,
        corelib::order::create_order,
        corelib::order::list_orders,
        corelib::order::get_order_by_id,
    ),
    components(
        schemas(
            RegisterInput, LoginInput, LoginResponse, UserInfo,
            CreateConcert,
            CreateTicket, Ticket,
            CreateOrder, OrderQuery, OrderView
        )
    ),
    tags(
        (name = "Ticketing API", description = "演唱會票務系統 API")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub struct ApiDoc;
