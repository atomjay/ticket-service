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

### 領域驅動設計 (DDD) 實踐

本專案採用領域驅動設計 (Domain-Driven Design, DDD) 作為核心架構理念，通過將複雜的業務邏輯組織成清晰的領域模型，實現了高內聚、低耦合的系統設計。

#### 1. 分層架構

專案遵循嚴格的分層架構，確保關注點分離：

- **領域層 (Domain Layer)**：核心業務邏輯的所在地，不依賴於其他層
- **應用層 (Application Layer)**：協調領域對象完成用戶請求，實現業務用例
- **基礎設施層 (Infrastructure Layer)**：提供技術實現，如資料庫存取
- **介面層 (Interface/API Layer)**：處理用戶請求，將其轉換為應用層的調用

#### 2. 領域模型

專案中的領域模型反映了票務系統的核心業務概念：

- **實體 (Entities)**：如 `Concert`、`Ticket`、`Order` 等具有唯一標識符的模型
- **值對象 (Value Objects)**：如 `CreateOrder`、`OrderQuery` 等不需要唯一標識的對象
- **聚合 (Aggregates)**：如 `OrderView` 聚合了訂單、票券和演唱會的信息

例如，訂單相關的領域模型：

```rust
// 訂單實體
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ticket_id: Uuid,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
}

// 創建訂單值對象
pub struct CreateOrder {
    pub ticket_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

// 訂單視圖（聚合）
pub struct OrderView {
    pub id: Uuid,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub ticket_type: String,
    pub price: f64,
    pub concert_title: String,
    pub concert_date: NaiveDateTime,
}
```

#### 3. 儲存庫模式 (Repository Pattern)

儲存庫模式是 DDD 的核心模式之一，在專案中通過以下方式實現：

- **儲存庫介面**：在領域層定義抽象介面，如 `ConcertRepository`、`OrderRepository`
- **儲存庫實現**：在基礎設施層實現這些介面，如 `PgConcertRepository`、`PgOrderRepository`

例如，演唱會儲存庫介面：

```rust
#[async_trait]
pub trait ConcertRepository: Send + Sync {
    /// 根據 ID 查找演唱會
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Concert>, AppError>;
    
    /// 獲取所有演唱會
    async fn find_all(&self) -> Result<Vec<Concert>, AppError>;
    
    /// 創建新演唱會
    async fn create(&self, input: &CreateConcert) -> Result<Concert, AppError>;
}
```

#### 4. 依賴倒置原則 (DIP)

專案通過以下方式實現依賴倒置原則：

- 高層模組（應用服務）依賴於抽象（儲存庫介面），而非具體實現
- 通過構造函數注入依賴，使用 `Arc<dyn Repository>` 實現依賴的共享和多線程安全

例如，演唱會服務的實現：

```rust
pub struct ConcertService {
    concert_repository: Arc<dyn ConcertRepository>,
}

impl ConcertService {
    pub fn new(concert_repository: Arc<dyn ConcertRepository>) -> Self {
        Self { concert_repository }
    }
    
    // 業務方法...
}
```

#### 5. 通用語言 (Ubiquitous Language)

專案中使用與票務領域相關的術語，建立了開發人員和領域專家都能理解的通用語言：

- 使用領域專家和開發人員都能理解的術語：`Concert`、`Ticket`、`Order` 等
- 代碼註釋和變數命名反映了業務概念，如 `ticket_type`、`concert_title`、`venue` 等

#### 6. 邊界上下文 (Bounded Contexts)

專案通過模組化結構實現了邊界上下文的概念：

- 將不同的業務領域分隔為獨立的模組：`auth`、`concert`、`ticket`、`order`
- 每個模組有自己的模型和儲存庫
- 通過應用層服務協調不同上下文之間的交互

### 詳細執行流程

以下是應用程式啟動時的詳細執行流程：

```
1. main.rs (程式入口點)
   │
   ├─> 讀取環境變數配置 (AppConfig::from_env())
   │    │
   │    └─> 從 .env 文件或系統環境變數獲取：
   │        - DATABASE_URL
   │        - JWT_SECRET
   │        - PORT
   │
   ├─> 初始化日誌系統 (tracing_subscriber::fmt::init())
   │
   ├─> 初始化資料庫連接池 (init_pool())
   │    │
   │    └─> 從 DATABASE_URL 創建 PostgreSQL 連接池
   │        - 設置最大連接數 (10)
   │        - 設置連接超時 (3秒)
   │
   ├─> 初始化資料庫存儲庫 (Repositories)
   │    │
   │    ├─> PgUserRepository::new(pool.clone())
   │    ├─> PgConcertRepository::new(pool.clone())
   │    ├─> PgTicketRepository::new(pool.clone())
   │    └─> PgOrderRepository::new(pool.clone())
   │
   ├─> 初始化應用服務 (Services)
   │    │
   │    ├─> AuthService::new(user_repository, jwt_secret)
   │    ├─> ConcertService::new(concert_repository)
   │    ├─> TicketService::new(ticket_repository, concert_repository)
   │    └─> OrderService::new(order_repository, ticket_repository)
   │
   ├─> 創建 API 路由 (create_router())
   │    │
   │    ├─> 創建 AppState (包含所有服務的引用)
   │    │
   │    ├─> 定義 API 端點
   │    │    ├─> /auth/* (認證相關)
   │    │    ├─> /concerts (演唱會相關)
   │    │    ├─> /tickets (票券相關)
   │    │    └─> /orders/* (訂單相關)
   │    │
   │    ├─> 添加 Swagger UI (/docs)
   │    │
   │    └─> 添加中間件
   │         ├─> CORS (跨域資源共享)
   │         ├─> Trace (請求追蹤)
   │         └─> RequestBodyLimit (請求體大小限制)
   │
   └─> 啟動 HTTP 服務器
        │
        ├─> 綁定 TCP 監聽器 (127.0.0.1:PORT)
        │
        └─> 開始處理 HTTP 請求 (axum::serve)
```

### 請求處理流程

當收到 HTTP 請求時，處理流程如下：

```
HTTP 請求
   │
   ├─> 中間件處理 (Middleware)
   │    │
   │    ├─> CORS 檢查
   │    ├─> 請求體大小檢查
   │    └─> 認證檢查 (對於需要認證的端點)
   │
   ├─> 路由匹配 (Router)
   │    │
   │    └─> 將請求路由到對應的處理器 (Handler)
   │
   ├─> 處理器處理請求 (Handler)
   │    │
   │    ├─> 解析請求參數
   │    ├─> 驗證輸入數據
   │    └─> 調用對應的服務方法
   │
   ├─> 服務層處理業務邏輯 (Service)
   │    │
   │    ├─> 實現業務規則
   │    └─> 調用存儲庫進行數據操作
   │
   ├─> 存儲庫訪問資料庫 (Repository)
   │    │
   │    ├─> 執行 SQL 查詢
   │    └─> 將資料庫記錄轉換為領域模型
   │
   └─> 返回 HTTP 響應
        │
        ├─> 序列化響應數據 (JSON)
        └─> 設置適當的 HTTP 狀態碼
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

MIT License

Copyright (c) 2025 [Atom Jay](https://github.com/atomjay)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
