# 個人的學習Rust專案

## 票務系統 (Ticket Service)

這是一個使用 Rust 和 Axum 框架開發的演唱會票務系統 API 服務。系統提供了用戶認證、演唱會管理、票券管理和訂單處理等功能。

## 功能特點

- **用戶管理**：註冊、登入和身份驗證
- **演唱會管理**：創建和查詢演唱會信息
- **票券管理**：創建和查詢票券
- **訂單系統**：創建訂單、查詢訂單列表和訂單詳情

## 技術棧

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

```
DATABASE_URL=postgres://username:password@localhost/ticket_service
JWT_SECRET=your_jwt_secret
PORT=3000
```

### 運行步驟

1. 克隆專案
   ```
   git clone <repository-url>
   cd ticket-service
   ```

2. 設置資料庫
   ```
   # 創建資料庫
   createdb ticket_service
   
   # 運行遷移
   sqlx database create
   sqlx migrate run
   ```

3. 編譯並運行
   ```
   cargo run
   ```

4. 訪問 API 文檔
   ```
   http://localhost:3000/docs
   ```

## API 端點

### 認證 API

- `POST /api/users/register` - 用戶註冊
- `POST /api/users/login` - 用戶登入
- `GET /api/users/me` - 獲取當前用戶信息

### 演唱會 API

- `POST /api/concerts` - 創建演唱會 (管理員)
- `GET /api/concerts` - 獲取演唱會列表

### 票券 API

- `POST /api/tickets` - 創建票券 (管理員)
- `GET /api/tickets` - 獲取票券列表

### 訂單 API

- `POST /api/orders` - 創建訂單
- `GET /api/orders` - 獲取訂單列表
- `GET /api/orders/:id` - 獲取訂單詳情

## 開發指南

### 添加新功能

1. 在 `domain` 層定義模型和存儲庫介面
2. 在 `infrastructure` 層實現存儲庫
3. 在 `application` 層實現服務邏輯
4. 在 `api/handlers` 中添加處理器
5. 在 `api/routes.rs` 中註冊路由

### 代碼風格

- 使用 `rustfmt` 格式化代碼
- 遵循 Rust 命名慣例
- 為公共 API 添加文檔註釋

## 授權

[MIT License](LICENSE)
