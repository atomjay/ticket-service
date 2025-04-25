//! HTTP 相關的基礎設施

/// HTTP 相關的工具函數和結構
pub mod utils {
    use axum::http::StatusCode;
    
    /// 將錯誤轉換為 HTTP 狀態碼
    pub fn map_error<E: std::fmt::Debug>(err: E) -> StatusCode {
        tracing::error!("發生錯誤: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
