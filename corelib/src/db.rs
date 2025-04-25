use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn init_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
