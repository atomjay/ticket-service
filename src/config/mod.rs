use std::env;

/// 應用程式配置
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// 資料庫連接字符串
    pub database_url: String,
    /// JWT 密鑰
    pub jwt_secret: String,
    /// 服務器端口
    pub port: u16,
}

impl AppConfig {
    /// 從環境變數加載配置
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL 必須設置"),
            jwt_secret: env::var("SECRET").expect("SECRET 必須設置"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT 必須是有效的數字"),
        }
    }
}
