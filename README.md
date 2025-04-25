# 我的 Rust 學習專案：票務系統 (Ticket Service)

這是我在學習 Rust 過程中開發的演唱會票務系統 API 服務。這個專案旨在幫助我深入理解 Rust 語言特性、Web 開發模式以及領域驅動設計 (DDD) 的實踐。

## 學習目標

通過這個專案，我希望學習以下內容：

- Rust 語言的核心概念（所有權、借用、生命週期等）
- 使用 Axum 框架進行 Web 開發
- 實現領域驅動設計 (DDD) 和分層架構
- 使用 SQLx 進行資料庫操作
- JWT 認證和安全性實踐
- 非同步程式設計
- 錯誤處理和日誌記錄
- API 文檔生成

## 功能特點

- **用戶管理**：註冊、登入和身份驗證
- **演唱會管理**：創建和查詢演唱會信息
- **票券管理**：創建和查詢票券
- **訂單系統**：創建訂單、查詢訂單列表和訂單詳情

## 技術棧

- **語言**：Rust 2024 Edition
- **框架**：Axum (基於 Tokio 的 Rust Web 框架)
- **資料庫**：PostgreSQL 與 SQLx
- **認證**：JWT (JSON Web Tokens)
- **密碼加密**：Argon2
- **API 文檔**：Utoipa (OpenAPI/Swagger)
- **序列化/反序列化**：Serde
- **日誌**：Tracing

## 系統架構

本專案採用領域驅動設計 (DDD) 和分層架構，結構清晰，便於維護和擴展。

### 架構流程圖

```
main.rs (入口點)
  │
  ├─> config/ (配置)
  │     └─> AppConfig (從環境變數讀取配置)
  │
  ├─> infrastructure/ (基礎設施層)
  │     ├─> database/
  │     │     ├─> connection.rs (資料庫連接)
  │     │     ├─> migrations/ (資料庫遷移腳本)
  │     │     └─> repositories/ (資料庫存儲庫實現)
  │     │           ├─> user_repository.rs
  │     │           ├─> concert_repository.rs
  │     │           ├─> ticket_repository.rs
  │     │           └─> order_repository.rs
  │     └─> http.rs (HTTP 相關工具函數)
  │
  ├─> domain/ (領域層)
  │     ├─> auth/
  │     │     ├─> model/ (用戶和認證相關模型)
  │     │     └─> repository.rs (用戶存儲庫介面)
  │     ├─> concert/
  │     │     ├─> model/ (演唱會相關模型)
  │     │     └─> repository.rs (演唱會存儲庫介面)
  │     ├─> ticket/
  │     │     ├─> model/ (票券相關模型)
  │     │     └─> repository.rs (票券存儲庫介面)
  │     └─> order/
  │           ├─> model/ (訂單相關模型)
  │           └─> repository.rs (訂單存儲庫介面)
  │
  ├─> application/ (應用層)
  │     ├─> auth/
  │     │     └─> service.rs (認證服務)
  │     ├─> concert/
  │     │     └─> service.rs (演唱會服務)
  │     ├─> ticket/
  │     │     └─> service.rs (票券服務)
  │     └─> order/
  │           └─> service.rs (訂單服務)
  │
  ├─> api/ (API 層)
  │     ├─> docs/ (API 文檔)
  │     ├─> handlers/ (請求處理器)
  │     │     ├─> auth_handler.rs
  │     │     ├─> concert_handler.rs
  │     │     ├─> ticket_handler.rs
  │     │     └─> order_handler.rs
  │     ├─> middleware/ (中間件)
  │     │     └─> auth.rs (認證中間件)
  │     └─> routes.rs (路由定義)
  │
  └─> utils/ (工具函數)
        └─> error.rs (錯誤處理)
```

### 請求流程

1. **客戶端請求** → `main.rs` (服務器入口點)
2. **路由匹配** → `api/routes.rs` (將請求路由到對應的處理器)
3. **中間件處理** → `api/middleware/` (如認證檢查)
4. **處理器處理請求** → `api/handlers/` (接收請求並調用服務)
5. **業務邏輯處理** → `application/*/service.rs` (處理業務邏輯)
6. **資料存取** → `infrastructure/database/repositories/` (與資料庫交互)
7. **返回響應** → 處理結果返回給客戶端

## 數據庫結構

系統使用 PostgreSQL 資料庫，主要包含以下表：

- **users**：用戶信息
- **concerts**：演唱會信息
- **tickets**：票券信息
- **orders**：訂單信息

## 開始使用

### 前置條件

- Rust (2024 Edition)
- PostgreSQL

### 環境變數

創建 `.env` 文件並設置以下環境變數：

```bash
DATABASE_URL=postgres://username:password@localhost:5432/ticket_service
SECRET=your_jwt_secret
PORT=8080
```

### 運行步驟

1. Clone 專案

```bash
git clone https://github.com/atomjay/ticket-service.git
cd ticket-service
```

2. 設置資料庫

```bash
# 創建資料庫
createdb ticket_service

# 運行遷移
sqlx migrate run
```

3. 編譯並運行

```bash
cargo run
```

4. 訪問 API 文檔

```
http://localhost:8080/docs
```

## API 端點

### 認證 API

- `POST /auth/register` - 用戶註冊
- `POST /auth/login` - 用戶登入
- `GET /auth/me` - 獲取當前用戶信息

### 演唱會 API

- `POST /concerts` - 創建演唱會 (管理員)
- `GET /concerts` - 獲取演唱會列表

### 票券 API

- `POST /tickets` - 創建票券 (管理員)
- `GET /tickets` - 獲取票券列表

### 訂單 API

- `POST /orders` - 創建訂單
- `GET /orders` - 獲取訂單列表
- `GET /orders/:order_id` - 獲取訂單詳情

## 學習筆記

### Rust 特性應用

- **所有權系統**：通過使用引用計數（Arc）在服務間共享資源
- **錯誤處理**：使用 `Result` 和自定義錯誤類型 `AppError`
- **非同步程式設計**：使用 `async/await` 實現非阻塞操作
- **特徵（Traits）**：定義存儲庫介面和實現

### 遇到的挑戰

1. **SQLx 離線模式**：學習如何使用 SQLx 的離線模式，避免在編譯時需要連接資料庫
2. **認證中間件**：實現 JWT 認證和權限檢查
3. **資料庫遷移**：管理資料庫結構的變更
4. **錯誤處理**：設計合理的錯誤處理機制

### 未來改進

- 添加更多的單元測試和整合測試
- 實現更完善的日誌記錄
- 添加更多的業務邏輯，如票券庫存管理
- 優化性能和安全性

## 參考資源

- [Rust 官方文檔](https://doc.rust-lang.org/book/)
- [Axum 文檔](https://docs.rs/axum/latest/axum/)
- [SQLx 文檔](https://docs.rs/sqlx/latest/sqlx/)
- [領域驅動設計 (DDD) 實踐](https://martinfowler.com/bliki/DomainDrivenDesign.html)

## 授權

[MIT License](LICENSE)
