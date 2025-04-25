// 引入標準庫中的環境變量模塊，用於讀取系統環境變量
use std::env;

/// 應用程式配置
/// 這個結構體包含了應用程序運行所需的所有配置信息
/// #[derive(Clone, Debug)] 是 Rust 的屬性標記，自動為結構體實現 Clone 和 Debug 特性
/// Clone 允許我們創建結構體的副本，Debug 允許我們以調試格式打印結構體
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// 資料庫連接字符串
    /// 這是連接到 PostgreSQL 數據庫所需的 URL，格式通常為：
    /// postgres://用戶名:密碼@主機:端口/數據庫名
    pub database_url: String,
    
    /// JWT 密鑰
    /// JWT (JSON Web Token) 是用於用戶認證的令牌
    /// 這個密鑰用於簽名和驗證 JWT，保護用戶身份信息
    pub jwt_secret: String,
    
    /// 服務器端口
    /// 應用程序將在這個端口上監聽 HTTP 請求
    /// u16 是一個 16 位無符號整數，範圍是 0-65535
    /// 常用的 HTTP 端口如 8080, 3000 等都在這個範圍內
    pub port: u16,
}

impl AppConfig {
    /// 從環境變數加載配置
    /// 這個方法讀取系統環境變量或 .env 文件中的配置值
    pub fn from_env() -> Self {
        // 加載 .env 文件中的環境變量
        // dotenvy 是一個庫，用於從 .env 文件讀取環境變量
        // ok() 方法將 Result 轉換為 Option，忽略可能的錯誤
        // 這意味著如果 .env 文件不存在，程序不會崩潰，而是繼續使用系統環境變量
        dotenvy::dotenv().ok();

        // 創建並返回配置實例
        Self {
            // 讀取 DATABASE_URL 環境變量
            // expect 在變量不存在時會終止程序並顯示錯誤信息
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL 必須設置"),
            
            // 讀取 SECRET 環境變量，用於 JWT 簽名
            jwt_secret: env::var("SECRET").expect("SECRET 必須設置"),
            
            // 讀取 PORT 環境變量
            // unwrap_or_else 提供了一個默認值（3000），如果環境變量不存在
            // parse 將字符串轉換為數字（u16）
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT 必須是有效的數字"),
        }
    }
}
