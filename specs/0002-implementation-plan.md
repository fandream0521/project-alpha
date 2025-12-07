# Ticket管理系统实现计划

## 文档信息
- **创建日期**: 2025-12-07
- **基于规格**: [0001-spec.md](./0001-spec.md)
- **计划版本**: v1.0

## 1. 实施概述

本实现计划基于[0001-spec.md](./0001-spec.md)中的需求与设计文档，将项目分为4个主要阶段，预计总开发周期4-5周。每个阶段都有明确的交付目标和验收标准。

### 1.1 技术栈确认
- **后端**: Rust + axum + sqlx + PostgreSQL
- **前端**: React 19 + TypeScript + Vite + Tailwind CSS + Shadcn/ui
- **部署**: Nginx + systemd（Linux环境）

## 2. 实施阶段（7个阶段，共7-8周）

### 阶段一：环境搭建与项目初始化（第1周）

#### 1.1 环境准备（第1周周一、周二）

**任务清单：**

1. **开发工具安装**
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustup update stable

   # 安装 Node.js
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 20
   nvm use 20

   # 安装 PostgreSQL
   # Ubuntu/Debian:
   sudo apt update && sudo apt install postgresql postgresql-contrib
   # macOS:
   brew install postgresql
   brew services start postgresql
   ```

2. **IDE配置**
   - VS Code 扩展安装：
     - Rust Analyzer
     - TypeScript and JavaScript Language Features
     - Tailwind CSS IntelliSense
     - ES7+ React/Redux/React-Native snippets

3. **数据库准备**
   ```sql
   -- 创建数据库
   CREATE DATABASE ticket_db;
   CREATE USER ticket_user WITH PASSWORD 'your_secure_password';
   GRANT ALL PRIVILEGES ON DATABASE ticket_db TO ticket_user;
   -- 测试连接
   \c ticket_db;
   SELECT version();
   ```

#### 1.2 项目初始化（第1周周三、周四）

**任务清单：**

1. **后端项目创建**
   ```bash
   cargo new ticket-backend --name ticket-backend
   cd ticket-backend
   git init
   ```

2. **前端项目创建**
   ```bash
   npm create vite@latest ticket-frontend -- --template react-ts
   cd ticket-frontend
   npm install
   git init
   ```

3. **Git仓库配置**
   ```bash
   # 在根目录
   git init
   echo "node_modules/" > .gitignore
   echo "target/" >> .gitignore
   echo ".env" >> .gitignore
   ```

#### 1.3 基础依赖配置（第1周周五）

**后端依赖（Cargo.toml）：**
```toml
[package]
name = "ticket-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono", "migrate"] }
tokio = { version = "1.41", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1.0"
uuid = { version = "1.11", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15"
```

**前端依赖（package.json）：**
```json
{
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "typescript": "^5.6.0",
    "axios": "^1.7.0",
    "zustand": "^5.0.0",
    "react-router-dom": "^7.1.0",
    "@tanstack/react-query": "^5.64.0",
    "react-hook-form": "^7.55.0",
    "@hookform/resolvers": "^3.10.0",
    "zod": "^3.24.0",
    "clsx": "^2.2.0",
    "tailwind-merge": "^2.5.0",
    "lucide-react": "^0.453.0",
    "react-hot-toast": "^2.5.0",
    "date-fns": "^4.1.0"
  }
}
```

#### 1.4 阶段一验收标准
- [ ] 所有开发工具安装完成
- [ ] 数据库创建成功，可以连接
- [ ] Rust和Node.js项目可以正常创建
- [ ] 基础依赖安装成功
- [ ] Git仓库初始化完成

---

### 阶段二：后端基础架构实现（第2周）

#### 2.1 项目结构搭建（第2周周一）

**创建目录结构：**
```bash
cd ticket-backend/src
mkdir -p {config,models/{ticket,tag},handlers,tickets,tags,services,repositories,utils}
touch {main.rs,config/mod.rs,models/mod.rs,handlers/mod.rs,services/mod.rs,repositories/mod.rs,utils/mod.rs,errors.rs}
```

#### 2.2 数据库设计与迁移（第2周周二、周三）

**创建迁移文件：**
```bash
# 安装 sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# 初始化迁移
sqlx migrate add create_tickets_table
sqlx migrate add create_tags_table
sqlx migrate add create_ticket_tags_table
sqlx migrate add create_indexes
```

**迁移脚本内容：**

1. `001_create_tickets_table.sql`
```sql
CREATE TABLE tickets (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'completed')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

2. `002_create_tags_table.sql`
```sql
CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    color VARCHAR(7) NOT NULL DEFAULT '#3B82F6',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

#### 2.3 数据模型定义（第2周周四）

**models/ticket.rs：**
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub tag_ids: Option<Vec<i64>>,
}
```

#### 2.4 基础服务层实现（第2周周五）

**services/tickets.rs：**
```rust
use crate::models::ticket::{Ticket, CreateTicket, UpdateTicket};
use crate::repositories::tickets::TicketRepository;
use sqlx::PgPool;
use anyhow::Result;

pub struct TicketService {
    repository: TicketRepository,
}

impl TicketService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: TicketRepository::new(pool),
        }
    }

    pub async fn create_ticket(&self, data: CreateTicket) -> Result<Ticket> {
        // 业务逻辑验证
        if data.title.trim().is_empty() {
            return Err(anyhow::anyhow!("标题不能为空"));
        }
        self.repository.create(data).await
    }

    pub async fn get_tickets(
        &self,
        page: u32,
        limit: u32,
        status: Option<String>,
    ) -> Result<(Vec<Ticket>, i64)> {
        self.repository.list(page, limit, status).await
    }

    // 其他方法...
}
```

#### 2.5 阶段二验收标准
- [ ] 数据库迁移成功执行
- [ ] 数据模型定义完整
- [ ] 服务层基础结构搭建完成
- [ ] 代码组织清晰，符合分层架构

---

### 阶段三：后端API接口实现（第3周）

#### 3.1 Repository层实现（第3周周一、周二）

**repositories/tickets.rs：**
```rust
use sqlx::PgPool;
use crate::models::ticket::{Ticket, CreateTicket, UpdateTicket};

pub struct TicketRepository {
    pool: PgPool,
}

impl TicketRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreateTicket) -> anyhow::Result<Ticket> {
        let ticket = sqlx::query_as!(
            Ticket,
            r#"
            INSERT INTO tickets (title, description)
            VALUES ($1, $2)
            RETURNING *
            "#,
            data.title,
            data.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ticket)
    }

    pub async fn list(
        &self,
        page: u32,
        limit: u32,
        status: Option<String>,
    ) -> anyhow::Result<(Vec<Ticket>, i64)> {
        let offset = (page - 1) * limit;

        let tickets = if let Some(status) = status {
            sqlx::query_as!(
                Ticket,
                r#"
                SELECT * FROM tickets
                WHERE status = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                status,
                limit as i64,
                offset as i64
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as!(
                Ticket,
                r#"
                SELECT * FROM tickets
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                limit as i64,
                offset as i64
            )
            .fetch_all(&self.pool)
            .await?
        };

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tickets"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        Ok((tickets, total))
    }

    // 其他 CRUD 方法...
}
```

#### 3.2 Handler层实现（第3周周三、周四）

**handlers/tickets.rs：**
```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use crate::models::ticket::{Ticket, CreateTicket, UpdateTicket};
use crate::services::tickets::TicketService;
use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct ListTicketsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TicketListResponse {
    pub data: Vec<Ticket>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: i64,
}

pub async fn list_tickets(
    State(service): State<TicketService>,
    Query(query): Query<ListTicketsQuery>,
) -> Result<Json<TicketListResponse>, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);

    let (tickets, total) = service
        .get_tickets(page, limit, query.status)
        .await?;

    Ok(Json(TicketListResponse {
        data: tickets,
        pagination: Pagination {
            page,
            limit,
            total,
        },
    }))
}

pub async fn create_ticket(
    State(service): State<TicketService>,
    Json(data): Json<CreateTicket>,
) -> Result<Json<Ticket>, AppError> {
    let ticket = service.create_ticket(data).await?;
    Ok(Json(ticket))
}

// 其他 handler 方法...
```

#### 3.3 路由配置（第3周周五）

**main.rs 关键部分：**
```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::CorsLayer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;

    // 运行迁移
    sqlx::migrate!("./migrations").run(&pool).await?;

    let ticket_service = Arc::new(crate::services::tickets::TicketService::new(pool.clone()));

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/tickets", get(handlers::tickets::list_tickets))
        .route("/api/v1/tickets", post(handlers::tickets::create_ticket))
        .route("/api/v1/tickets/:id", get(handlers::tickets::get_ticket))
        .route("/api/v1/tickets/:id", put(handlers::tickets::update_ticket))
        .route("/api/v1/tickets/:id", delete(handlers::tickets::delete_ticket))
        .layer(CorsLayer::permissive())
        .with_state(ticket_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
```

#### 3.4 阶段三验收标准
- [ ] 所有Ticket CRUD API实现完成
- [ ] API响应格式符合设计规范
- [ ] 错误处理机制完善
- [ ] 可以通过Postman或其他工具测试API
- [ ] 分页功能正常工作

---

### 阶段三+：后端API验证与测试（第3周末）

#### 3.5 HTTP REST API测试文件创建

在项目根目录创建 `http.rest` 文件，用于使用VS Code REST Client或其他HTTP客户端工具测试所有API接口。

**创建 http.rest 文件：**
```http
### Ticket管理系统API测试文件
### 使用方法：在VS Code中安装REST Client扩展，然后点击每个请求上方的"Send Request"

@baseUrl = http://localhost:8080
@contentType = application/json

### 1. 健康检查
GET {{baseUrl}}/health

###

### 2. Ticket相关API测试

#### 2.1 获取所有Tickets（基础）
GET {{baseUrl}}/api/v1/tickets

###

#### 2.2 获取所有Tickets（分页）
GET {{baseUrl}}/api/v1/tickets?page=1&limit=10

###

#### 2.3 获取Tickets（状态过滤）
GET {{baseUrl}}/api/v1/tickets?status=open

###

#### 2.4 获取Tickets（搜索）
GET {{baseUrl}}/api/v1/tickets?search=测试

###

#### 2.5 获取Tickets（复合查询）
GET {{baseUrl}}/api/v1/tickets?status=open&search=重要&page=1&limit=5

###

#### 2.6 创建新Ticket（仅标题）
POST {{baseUrl}}/api/v1/tickets
Content-Type: {{contentType}}

{
  "title": "测试Ticket - API创建"
}

###

#### 2.7 创建新Ticket（带描述和标签）
POST {{baseUrl}}/api/v1/tickets
Content-Type: {{contentType}}

{
  "title": "功能开发任务",
  "description": "实现用户登录功能，包括JWT认证和权限管理",
  "tag_ids": [1, 2]
}

###

#### 2.8 创建Ticket（空标题 - 错误测试）
POST {{baseUrl}}/api/v1/tickets
Content-Type: {{contentType}}

{
  "title": ""
}

###

### 动态变量：存储创建的Ticket ID
@ticketId = 1

#### 2.9 获取单个Ticket详情
GET {{baseUrl}}/api/v1/tickets/{{ticketId}}

###

#### 2.10 获取不存在的Ticket（404测试）
GET {{baseUrl}}/api/v1/tickets/99999

###

#### 2.11 更新Ticket（仅标题）
PUT {{baseUrl}}/api/v1/tickets/{{ticketId}}
Content-Type: {{contentType}}

{
  "title": "更新后的标题"
}

###

#### 2.12 更新Ticket（完整更新）
PUT {{baseUrl}}/api/v1/tickets/{{ticketId}}
Content-Type: {{contentType}}

{
  "title": "完整更新的Ticket",
  "description": "这是更新后的描述信息",
  "status": "in_progress",
  "tag_ids": [1, 3]
}

###

#### 2.13 更新Ticket状态为完成
PUT {{baseUrl}}/api/v1/tickets/{{ticketId}}
Content-Type: {{contentType}}

{
  "status": "completed"
}

###

#### 2.14 更新Ticket（部分更新 - 仅状态）
PUT {{baseUrl}}/api/v1/tickets/{{ticketId}}
Content-Type: {{contentType}}

{
  "status": "open"
}

###

#### 2.15 删除Ticket
DELETE {{baseUrl}}/api/v1/tickets/{{ticketId}}

###

#### 2.16 删除不存在的Ticket
DELETE {{baseUrl}}/api/v1/tickets/99999

###

### 3. 标签相关API测试

#### 3.1 获取所有标签
GET {{baseUrl}}/api/v1/tags

###

#### 3.2 创建新标签
POST {{baseUrl}}/api/v1/tags
Content-Type: {{contentType}}

{
  "name": "重要",
  "color": "#EF4444"
}

###

#### 3.3 创建标签（仅名称）
POST {{baseUrl}}/api/v1/tags
Content-Type: {{contentType}}

{
  "name": "Bug"
}

###

#### 3.4 创建标签（重复名称 - 错误测试）
POST {{baseUrl}}/api/v1/tags
Content-Type: {{contentType}}

{
  "name": "重要"
}

###

### 动态变量：存储创建的标签ID
@tagId = 1

#### 3.5 获取单个标签详情
GET {{baseUrl}}/api/v1/tags/{{tagId}}

###

#### 3.6 更新标签（仅名称）
PUT {{baseUrl}}/api/v1/tags/{{tagId}}
Content-Type: {{contentType}}

{
  "name": "紧急"
}

###

#### 3.7 更新标签（完整更新）
PUT {{baseUrl}}/api/v1/tags/{{tagId}}
Content-Type: {{contentType}}

{
  "name": "功能",
  "color": "#10B981"
}

###

#### 3.8 删除标签
DELETE {{baseUrl}}/api/v1/tags/{{tagId}}

###

#### 3.9 删除不存在的标签
DELETE {{baseUrl}}/api/v1/tags/99999

###

### 4. 高级搜索测试

#### 4.1 全文搜索
GET {{baseUrl}}/api/v1/tickets?search=开发

###

#### 4.2 多标签过滤
GET {{baseUrl}}/api/v1/tickets?tag_ids=1,2,3

###

#### 4.3 组合查询（状态+搜索）
GET {{baseUrl}}/api/v1/tickets?status=open&search=测试

###

#### 4.4 组合查询（标签+状态）
GET {{baseUrl}}/api/v1/tickets?tag_ids=1&status=completed

###

#### 4.5 复杂组合查询
GET {{baseUrl}}/api/v1/tickets?search=开发&tag_ids=1,2&status=open&page=1&limit=10

###

### 5. 边界测试

#### 5.1 大标题测试
POST {{baseUrl}}/api/v1/tickets
Content-Type: {{contentType}}

{
  "title": "这是一个非常非常长的标题，用于测试系统对长文本的处理能力，正常情况下应该能够正常处理但可能会有长度限制"
}

###

#### 5.2 特殊字符测试
POST {{baseUrl}}/api/v1/tickets
Content-Type: {{contentType}}

{
  "title": "测试特殊字符 !@#$%^&*()_+-=[]{}|;':\",./<>?",
  "description": "描述包含特殊字符：中文、English、123、!@#$%"
}

###

#### 5.3 SQL注入测试（应该被防御）
GET {{baseUrl}}/api/v1/tickets?search='; DROP TABLE tickets; --

###

#### 5.4 XSS测试（应该被转义）
POST {{baseUrl}}/api/v1/tickets
Content-Type: {{contentType}}

{
  "title": "<script>alert('XSS测试')</script>",
  "description": "包含HTML标签的描述：<b>加粗</b>和<em>斜体</em>"
}

###

### 6. 性能测试（批量数据）

#### 6.1 大分页测试
GET {{baseUrl}}/api/v1/tickets?page=1&limit=100

###

#### 6.2 远程分页测试
GET {{baseUrl}}/api/v1/tickets?page=999&limit=50

###

### 7. 并发测试建议
# 使用以下命令进行并发测试：
# 使用Apache Bench：
# ab -n 100 -c 10 http://localhost:8080/api/v1/tickets
#
# 使用curl进行简单并发：
# for i in {1..10}; do curl -X GET http://localhost:8080/api/v1/tickets & done; wait

### 8. 测试数据清理（可选）
# 清理所有测试数据
# DELETE FROM ticket_tags;
# DELETE FROM tickets;
# DELETE FROM tags;
# 注意：这会删除所有数据，请谨慎使用！

```

#### 3.6 API验证任务清单

**1. 安装REST Client工具**
```bash
# VS Code扩展
# - REST Client (humao.rest-client)
# - Thunder Client (rangav.vscode-thunder-client)
```

**2. 启动后端服务**
```bash
cd ticket-backend
cargo run
# 确保服务在 http://localhost:8080 运行
```

**3. 执行API测试**

**基础功能验证：**
- [ ] 健康检查接口正常响应
- [ ] 创建Ticket成功
- [ ] 获取Ticket列表正常
- [ ] 获取单个Ticket详情正常
- [ ] 更新Ticket功能正常
- [ ] 删除Ticket功能正常

**分页功能验证：**
- [ ] 默认分页参数生效
- [ ] 自定义分页参数生效
- [ ] 分页信息返回正确
- [ ] 边界值处理正确

**搜索功能验证：**
- [ ] 关键词搜索正常
- [ ] 状态过滤正常
- [ ] 组合查询正常
- [ ] 空搜索结果处理正确

**错误处理验证：**
- [ ] 404错误处理正确
- [ ] 400错误（参数错误）处理正确
- [ ] 错误响应格式统一
- [ ] 错误信息有意义

**标签功能验证：**
- [ ] 创建标签成功
- [ ] 更新标签成功
- [ ] 删除标签成功
- [ ] Ticket标签关联正常

**性能验证：**
- [ ] 响应时间在可接受范围内
- [ ] 并发请求处理正常
- [ ] 大数据量查询优化

#### 3.7 API测试验收标准
- [ ] 所有HTTP接口都能正常响应
- [ ] CRUD操作完整可用
- [ ] 错误处理机制完善
- [ ] API响应格式统一且正确
- [ ] 分页功能正常工作
- [ ] 搜索过滤功能正常
- [ ] 性能指标达到预期
- [ ] http.rest文件完整可用

---

### 阶段四：前端基础框架搭建（第4周）

#### 4.1 项目配置（第4周周一）

**Tailwind CSS配置：**
```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

**tailwind.config.js：**
```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

#### 4.2 基础UI组件（第4周周二、周三）

**src/components/ui/Button.tsx：**
```tsx
import { clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function Button({
  className,
  variant = "primary",
  size = "md",
  ...props
}: React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary" | "outline"
  size?: "sm" | "md" | "lg"
}) {
  return (
    <button
      className={twMerge(clsx(
        "inline-flex items-center justify-center rounded-md font-medium transition-colors",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2",
        {
          "bg-blue-600 text-white hover:bg-blue-700": variant === "primary",
          "bg-gray-100 text-gray-900 hover:bg-gray-200": variant === "secondary",
          "border border-gray-300 bg-white hover:bg-gray-50": variant === "outline",
        },
        {
          "h-8 px-3 text-sm": size === "sm",
          "h-10 px-4 py-2": size === "md",
          "h-12 px-6 text-lg": size === "lg",
        }
      ), className)}
      {...props}
    />
  )
}
```

**src/components/ui/Input.tsx：**
```tsx
import { forwardRef } from "react"
import { clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export const Input = forwardRef<HTMLInputElement, React.InputHTMLAttributes<HTMLInputElement>>(
  ({ className, type, ...props }, ref) => {
    return (
      <input
        type={type}
        className={twMerge(clsx(
          "flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm",
          "ring-offset-white file:border-0 file:bg-transparent file:text-sm file:font-medium",
          "placeholder:text-gray-500 focus-visible:outline-none focus-visible:ring-2",
          "focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
        ), className)}
        ref={ref}
        {...props}
      />
    )
  }
)
Input.displayName = "Input"
```

#### 4.3 API服务层（第4周周四）

**src/services/api.ts：**
```typescript
import axios from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080'

export const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器
api.interceptors.request.use(
  (config) => {
    console.log(`API Request: ${config.method?.toUpperCase()} ${config.url}`)
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
api.interceptors.response.use(
  (response) => {
    return response
  },
  (error) => {
    const message = error.response?.data?.error?.message || error.message || '请求失败'
    console.error('API Error:', message)
    return Promise.reject(new Error(message))
  }
)
```

**src/services/tickets.ts：**
```typescript
import { api } from './api'
import { Ticket, CreateTicketData, UpdateTicketData, TicketListResponse } from '../types/ticket'

export const ticketService = {
  async getTickets(params?: {
    page?: number
    limit?: number
    status?: string
    search?: string
  }): Promise<TicketListResponse> {
    const response = await api.get('/api/v1/tickets', { params })
    return response.data
  },

  async getTicket(id: number): Promise<Ticket> {
    const response = await api.get(`/api/v1/tickets/${id}`)
    return response.data
  },

  async createTicket(data: CreateTicketData): Promise<Ticket> {
    const response = await api.post('/api/v1/tickets', data)
    return response.data
  },

  async updateTicket(id: number, data: UpdateTicketData): Promise<Ticket> {
    const response = await api.put(`/api/v1/tickets/${id}`, data)
    return response.data
  },

  async deleteTicket(id: number): Promise<void> {
    await api.delete(`/api/v1/tickets/${id}`)
  },
}
```

#### 4.4 状态管理（第4周周五）

**src/store/ticketStore.ts：**
```typescript
import { create } from 'zustand'
import { devtools } from 'zustand/middleware'
import { Ticket, CreateTicketData, UpdateTicketData } from '../types/ticket'
import { ticketService } from '../services/tickets'

interface TicketStore {
  // State
  tickets: Ticket[]
  loading: boolean
  error: string | null
  pagination: {
    page: number
    limit: number
    total: number
  }

  // Actions
  fetchTickets: (params?: any) => Promise<void>
  createTicket: (data: CreateTicketData) => Promise<void>
  updateTicket: (id: number, data: UpdateTicketData) => Promise<void>
  deleteTicket: (id: number) => Promise<void>
  clearError: () => void
}

export const useTicketStore = create<TicketStore>()(
  devtools(
    (set, get) => ({
      // Initial state
      tickets: [],
      loading: false,
      error: null,
      pagination: {
        page: 1,
        limit: 20,
        total: 0,
      },

      // Actions
      fetchTickets: async (params) => {
        set({ loading: true, error: null })
        try {
          const response = await ticketService.getTickets(params)
          set({
            tickets: response.data,
            pagination: {
              page: response.pagination.page,
              limit: response.pagination.limit,
              total: response.pagination.total,
            },
            loading: false,
          })
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : '获取Ticket列表失败',
            loading: false,
          })
        }
      },

      createTicket: async (data) => {
        set({ loading: true, error: null })
        try {
          const newTicket = await ticketService.createTicket(data)
          set(state => ({
            tickets: [newTicket, ...state.tickets],
            loading: false,
          }))
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : '创建Ticket失败',
            loading: false,
          })
          throw error
        }
      },

      updateTicket: async (id, data) => {
        set({ loading: true, error: null })
        try {
          const updatedTicket = await ticketService.updateTicket(id, data)
          set(state => ({
            tickets: state.tickets.map(t => t.id === id ? updatedTicket : t),
            loading: false,
          }))
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : '更新Ticket失败',
            loading: false,
          })
          throw error
        }
      },

      deleteTicket: async (id) => {
        set({ loading: true, error: null })
        try {
          await ticketService.deleteTicket(id)
          set(state => ({
            tickets: state.tickets.filter(t => t.id !== id),
            loading: false,
          }))
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : '删除Ticket失败',
            loading: false,
          })
          throw error
        }
      },

      clearError: () => set({ error: null }),
    }),
    {
      name: 'ticket-store',
    }
  )
)
```

#### 4.5 阶段四验收标准
- [ ] Tailwind CSS配置完成并正常工作
- [ ] 基础UI组件库搭建完成
- [ ] API服务层封装完成
- [ ] 状态管理配置完成
- [ ] 前端项目结构清晰

---

### 阶段五：前端核心功能实现（第5周）

#### 5.1 Ticket卡片组件（第5周周一）

**src/components/TicketCard.tsx：**
```tsx
import { Ticket } from '../types/ticket'
import { formatDistanceToNow } from 'date-fns'
import { zhCN } from 'date-fns/locale'
import { Button } from './ui/Button'
import { Check, Edit2, Trash2, RotateCcw } from 'lucide-react'

interface TicketCardProps {
  ticket: Ticket
  onEdit: (ticket: Ticket) => void
  onDelete: (id: number) => void
  onToggleStatus: (ticket: Ticket) => void
}

export function TicketCard({ ticket, onEdit, onDelete, onToggleStatus }: TicketCardProps) {
  const isCompleted = ticket.status === 'completed'

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <h3 className={`text-lg font-medium ${isCompleted ? 'line-through text-gray-500' : 'text-gray-900'}`}>
            {ticket.title}
          </h3>
          {ticket.description && (
            <p className="mt-1 text-sm text-gray-600 line-clamp-2">
              {ticket.description}
            </p>
          )}
          <div className="mt-3 flex items-center gap-4 text-sm text-gray-500">
            <span>
              创建于 {formatDistanceToNow(new Date(ticket.created_at), {
                addSuffix: true,
                locale: zhCN
              })}
            </span>
            {isCompleted && (
              <span className="text-green-600 font-medium">已完成</span>
            )}
          </div>
        </div>

        <div className="flex items-center gap-2 ml-4">
          <Button
            variant="outline"
            size="sm"
            onClick={() => onToggleStatus(ticket)}
            title={isCompleted ? '重新打开' : '标记完成'}
          >
            {isCompleted ? <RotateCcw className="h-4 w-4" /> : <Check className="h-4 w-4" />}
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => onEdit(ticket)}
            title="编辑"
          >
            <Edit2 className="h-4 w-4" />
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => onDelete(ticket.id)}
            title="删除"
            className="text-red-600 hover:text-red-700"
          >
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  )
}
```

#### 5.2 Ticket表单组件（第5周周二）

**src/components/TicketForm.tsx：**
```tsx
import { useState, useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { Ticket, CreateTicketData, UpdateTicketData } from '../types/ticket'
import { Button } from './ui/Button'
import { Input } from './ui/Input'
import { X } from 'lucide-react'

const ticketSchema = z.object({
  title: z.string().min(1, '标题不能为空').max(255, '标题不能超过255个字符'),
  description: z.string().optional(),
})

type TicketFormData = z.infer<typeof ticketSchema>

interface TicketFormProps {
  ticket?: Ticket
  onSubmit: (data: CreateTicketData | UpdateTicketData) => Promise<void>
  onCancel: () => void
  loading?: boolean
}

export function TicketForm({ ticket, onSubmit, onCancel, loading }: TicketFormProps) {
  const {
    register,
    handleSubmit,
    formState: { errors },
    reset,
  } = useForm<TicketFormData>({
    resolver: zodResolver(ticketSchema),
    defaultValues: {
      title: ticket?.title || '',
      description: ticket?.description || '',
    },
  })

  useEffect(() => {
    if (ticket) {
      reset({
        title: ticket.title,
        description: ticket.description || '',
      })
    }
  }, [ticket, reset])

  const handleFormSubmit = async (data: TicketFormData) => {
    try {
      await onSubmit(data)
      if (!ticket) {
        reset() // 只在创建成功后重置表单
      }
    } catch (error) {
      // 错误由父组件处理
    }
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
      <div className="bg-white rounded-lg shadow-lg w-full max-w-md">
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="text-lg font-medium">
            {ticket ? '编辑 Ticket' : '创建新 Ticket'}
          </h2>
          <Button
            variant="outline"
            size="sm"
            onClick={onCancel}
            disabled={loading}
          >
            <X className="h-4 w-4" />
          </Button>
        </div>

        <form onSubmit={handleSubmit(handleFormSubmit)} className="p-4 space-y-4">
          <div>
            <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-1">
              标题 <span className="text-red-500">*</span>
            </label>
            <Input
              id="title"
              placeholder="输入Ticket标题"
              {...register('title')}
              disabled={loading}
              className={errors.title ? 'border-red-500' : ''}
            />
            {errors.title && (
              <p className="mt-1 text-sm text-red-600">{errors.title.message}</p>
            )}
          </div>

          <div>
            <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
              描述
            </label>
            <textarea
              id="description"
              rows={4}
              placeholder="输入Ticket描述（可选）"
              className="flex w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm ring-offset-white placeholder:text-gray-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              {...register('description')}
              disabled={loading}
            />
          </div>

          <div className="flex gap-3 pt-4">
            <Button
              type="submit"
              disabled={loading}
              className="flex-1"
            >
              {loading ? '保存中...' : (ticket ? '更新' : '创建')}
            </Button>
            <Button
              type="button"
              variant="outline"
              onClick={onCancel}
              disabled={loading}
              className="flex-1"
            >
              取消
            </Button>
          </div>
        </form>
      </div>
    </div>
  )
}
```

#### 5.3 Ticket列表页面（第5周周三、周四）

**src/pages/HomePage.tsx：**
```tsx
import { useState, useEffect } from 'react'
import { Plus, Search } from 'lucide-react'
import { Ticket } from '../types/ticket'
import { Button } from '../components/ui/Button'
import { Input } from '../components/ui/Input'
import { TicketCard } from '../components/TicketCard'
import { TicketForm } from '../components/TicketForm'
import { useTicketStore } from '../store/ticketStore'
import toast from 'react-hot-toast'

export function HomePage() {
  const [searchTerm, setSearchTerm] = useState('')
  const [showForm, setShowForm] = useState(false)
  const [editingTicket, setEditingTicket] = useState<Ticket | undefined>()

  const {
    tickets,
    loading,
    error,
    fetchTickets,
    createTicket,
    updateTicket,
    deleteTicket,
    clearError,
  } = useTicketStore()

  useEffect(() => {
    fetchTickets()
  }, [fetchTickets])

  useEffect(() => {
    if (error) {
      toast.error(error)
      clearError()
    }
  }, [error, clearError])

  const handleCreateTicket = async (data: any) => {
    try {
      await createTicket(data)
      setShowForm(false)
      toast.success('Ticket创建成功')
    } catch (error) {
      // 错误已在store中处理
    }
  }

  const handleUpdateTicket = async (data: any) => {
    if (!editingTicket) return

    try {
      await updateTicket(editingTicket.id, data)
      setEditingTicket(undefined)
      toast.success('Ticket更新成功')
    } catch (error) {
      // 错误已在store中处理
    }
  }

  const handleDeleteTicket = async (id: number) => {
    if (window.confirm('确定要删除这个Ticket吗？')) {
      try {
        await deleteTicket(id)
        toast.success('Ticket删除成功')
      } catch (error) {
        // 错误已在store中处理
      }
    }
  }

  const handleToggleStatus = async (ticket: Ticket) => {
    try {
      const newStatus = ticket.status === 'open' ? 'completed' : 'open'
      await updateTicket(ticket.id, { status: newStatus })
      toast.success(`Ticket已${newStatus === 'completed' ? '完成' : '重新打开'}`)
    } catch (error) {
      // 错误已在store中处理
    }
  }

  const filteredTickets = tickets.filter(ticket =>
    ticket.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
    ticket.description?.toLowerCase().includes(searchTerm.toLowerCase())
  )

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto px-4 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">Ticket管理系统</h1>
          <div className="flex gap-4">
            <div className="flex-1 relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <Input
                placeholder="搜索Tickets..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-10"
              />
            </div>
            <Button
              onClick={() => setShowForm(true)}
              className="flex items-center gap-2"
            >
              <Plus className="h-4 w-4" />
              创建Ticket
            </Button>
          </div>
        </div>

        {/* Loading State */}
        {loading && (
          <div className="text-center py-12">
            <div className="text-gray-500">加载中...</div>
          </div>
        )}

        {/* Ticket List */}
        {!loading && (
          <div className="space-y-4">
            {filteredTickets.length === 0 ? (
              <div className="text-center py-12">
                <p className="text-gray-500">
                  {searchTerm ? '没有找到匹配的Tickets' : '还没有Tickets，创建一个吧！'}
                </p>
              </div>
            ) : (
              filteredTickets.map((ticket) => (
                <TicketCard
                  key={ticket.id}
                  ticket={ticket}
                  onEdit={setEditingTicket}
                  onDelete={handleDeleteTicket}
                  onToggleStatus={handleToggleStatus}
                />
              ))
            )}
          </div>
        )}

        {/* Forms */}
        {showForm && (
          <TicketForm
            onSubmit={handleCreateTicket}
            onCancel={() => setShowForm(false)}
            loading={loading}
          />
        )}

        {editingTicket && (
          <TicketForm
            ticket={editingTicket}
            onSubmit={handleUpdateTicket}
            onCancel={() => setEditingTicket(undefined)}
            loading={loading}
          />
        )}
      </div>
    </div>
  )
}
```

#### 5.4 主应用集成（第5周周五）

**src/App.tsx：**
```tsx
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { Toaster } from 'react-hot-toast'
import { HomePage } from './pages/HomePage'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60 * 1000, // 1 minute
      retry: 1,
    },
  },
})

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="App">
        <HomePage />
        <Toaster
          position="top-right"
          toastOptions={{
            duration: 4000,
            style: {
              background: '#363636',
              color: '#fff',
            },
            success: {
              duration: 3000,
              iconTheme: {
                primary: '#10b981',
                secondary: '#fff',
              },
            },
            error: {
              duration: 5000,
              iconTheme: {
                primary: '#ef4444',
                secondary: '#fff',
              },
            },
          }}
        />
      </div>
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}

export default App
```

#### 5.5 阶段五验收标准
- [ ] Ticket卡片组件显示正确
- [ ] 创建/编辑Ticket表单功能完整
- [ ] Ticket列表页面可以正常显示
- [ ] 所有CRUD操作正常工作
- [ ] Toast通知正常显示
- [ ] 搜索功能正常工作

---

### 阶段六：标签系统实现（第6周）

#### 6.1 标签后端实现（第6周周一、周二）

**models/tag.rs：**
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagWithCount {
    #[serde(flatten)]
    pub tag: Tag,
    pub ticket_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub color: Option<String>,
}
```

**handlers/tags.rs：**
```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use crate::models::tag::{Tag, CreateTag, UpdateTag, TagWithCount};
use crate::services::tags::TagService;
use crate::errors::AppError;

pub async fn list_tags(
    State(service): State<TagService>,
) -> Result<Json<Vec<TagWithCount>>, AppError> {
    let tags = service.get_tags_with_count().await?;
    Ok(Json(tags))
}

pub async fn create_tag(
    State(service): State<TagService>,
    Json(data): Json<CreateTag>,
) -> Result<Json<Tag>, AppError> {
    let tag = service.create_tag(data).await?;
    Ok(Json(tag))
}

pub async fn update_tag(
    State(service): State<TagService>,
    Path(id): Path<i64>,
    Json(data): Json<UpdateTag>,
) -> Result<Json<Tag>, AppError> {
    let tag = service.update_tag(id, data).await?;
    Ok(Json(tag))
}

pub async fn delete_tag(
    State(service): State<TagService>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    service.delete_tag(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

#### 6.2 标签前端组件（第6周周三）

**src/components/Tag.tsx：**
```tsx
import { Tag as TagType } from '../types/tag'
import { X } from 'lucide-react'
import { clsx } from 'clsx'

interface TagProps {
  tag: TagType
  removable?: boolean
  onRemove?: (id: number) => void
  size?: 'sm' | 'md'
}

export function Tag({ tag, removable, onRemove, size = 'md' }: TagProps) {
  const sizeClasses = {
    sm: 'px-2 py-0.5 text-xs',
    md: 'px-2.5 py-1 text-sm',
  }

  return (
    <span
      className={clsx(
        'inline-flex items-center gap-1 rounded-full font-medium',
        sizeClasses[size],
      )}
      style={{
        backgroundColor: `${tag.color}20`,
        color: tag.color,
        border: `1px solid ${tag.color}40`
      }}
    >
      {tag.name}
      {removable && onRemove && (
        <button
          onClick={() => onRemove(tag.id)}
          className="hover:bg-black/10 rounded-full p-0.5 transition-colors"
        >
          <X className="h-3 w-3" />
        </button>
      )}
    </span>
  )
}
```

**src/components/TagSelector.tsx：**
```tsx
import { useState, useEffect } from 'react'
import { Tag as TagType } from '../types/tag'
import { Tag } from './Tag'
import { ChevronDown, X } from 'lucide-react'
import { useTagStore } from '../store/tagStore'

interface TagSelectorProps {
  selectedTagIds: number[]
  onChange: (tagIds: number[]) => void
  placeholder?: string
}

export function TagSelector({ selectedTagIds, onChange, placeholder = '选择标签' }: TagSelectorProps) {
  const [isOpen, setIsOpen] = useState(false)
  const { tags, fetchTags } = useTagStore()

  useEffect(() => {
    fetchTags()
  }, [fetchTags])

  const selectedTags = tags.filter(tag => selectedTagIds.includes(tag.id))
  const availableTags = tags.filter(tag => !selectedTagIds.includes(tag.id))

  const handleTagSelect = (tagId: number) => {
    onChange([...selectedTagIds, tagId])
    setIsOpen(false)
  }

  const handleTagRemove = (tagId: number) => {
    onChange(selectedTagIds.filter(id => id !== tagId))
  }

  return (
    <div className="relative">
      {/* Selected Tags Display */}
      <div
        className="min-h-10 px-3 py-2 border border-gray-300 rounded-md bg-white cursor-text flex flex-wrap gap-2 items-center"
        onClick={() => setIsOpen(!isOpen)}
      >
        {selectedTags.length === 0 ? (
          <span className="text-gray-500">{placeholder}</span>
        ) : (
          selectedTags.map(tag => (
            <Tag
              key={tag.id}
              tag={tag}
              removable
              onRemove={handleTagRemove}
              size="sm"
            />
          ))
        )}
        <ChevronDown className="h-4 w-4 text-gray-400 ml-auto" />
      </div>

      {/* Dropdown */}
      {isOpen && (
        <div className="absolute z-10 mt-1 w-full bg-white border border-gray-300 rounded-md shadow-lg">
          <div className="max-h-60 overflow-auto py-1">
            {availableTags.length === 0 ? (
              <div className="px-3 py-2 text-sm text-gray-500">
                没有可用的标签
              </div>
            ) : (
              availableTags.map(tag => (
                <button
                  key={tag.id}
                  className="w-full px-3 py-2 text-left hover:bg-gray-50 flex items-center gap-2"
                  onClick={() => handleTagSelect(tag.id)}
                >
                  <Tag tag={tag} size="sm" />
                  <span className="text-sm text-gray-600">
                    ({tag.ticket_count} 个tickets)
                  </span>
                </button>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  )
}
```

#### 6.3 标签管理页面（第6周周四）

**src/pages/TagManagePage.tsx：**
```tsx
import { useState } from 'react'
import { Plus, Edit2, Trash2, Tag as TagIcon } from 'lucide-react'
import { Tag } from '../types/tag'
import { Button } from '../components/ui/Button'
import { Input } from '../components/ui/Input'
import { TagComponent } from '../components/Tag'
import { useTagStore } from '../store/tagStore'
import toast from 'react-hot-toast'

const PRESET_COLORS = [
  '#3B82F6', '#EF4444', '#10B981', '#F59E0B',
  '#8B5CF6', '#EC4899', '#6B7280', '#14B8A6',
  '#F97316', '#06B6D4', '#84CC16', '#A855F7'
]

export function TagManagePage() {
  const [showForm, setShowForm] = useState(false)
  const [editingTag, setEditingTag] = useState<Tag | undefined>()
  const [formData, setFormData] = useState({
    name: '',
    color: PRESET_COLORS[0],
  })

  const { tags, loading, fetchTags, createTag, updateTag, deleteTag } = useTagStore()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    try {
      if (editingTag) {
        await updateTag(editingTag.id, formData)
        toast.success('标签更新成功')
      } else {
        await createTag(formData)
        toast.success('标签创建成功')
      }

      setShowForm(false)
      setEditingTag(undefined)
      setFormData({ name: '', color: PRESET_COLORS[0] })
    } catch (error) {
      // 错误已在store中处理
    }
  }

  const handleEdit = (tag: Tag) => {
    setEditingTag(tag)
    setFormData({
      name: tag.name,
      color: tag.color,
    })
    setShowForm(true)
  }

  const handleDelete = async (id: number) => {
    if (window.confirm('确定要删除这个标签吗？删除后所有tickets上的此标签也会被移除。')) {
      try {
        await deleteTag(id)
        toast.success('标签删除成功')
      } catch (error) {
        // 错误已在store中处理
      }
    }
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto px-4 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">标签管理</h1>
          <Button
            onClick={() => {
              setEditingTag(undefined)
              setFormData({ name: '', color: PRESET_COLORS[0] })
              setShowForm(true)
            }}
            className="flex items-center gap-2"
          >
            <Plus className="h-4 w-4" />
            创建标签
          </Button>
        </div>

        {/* Tag Form */}
        {showForm && (
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
            <h2 className="text-lg font-medium mb-4">
              {editingTag ? '编辑标签' : '创建新标签'}
            </h2>
            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  标签名称
                </label>
                <Input
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="输入标签名称"
                  required
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  选择颜色
                </label>
                <div className="flex flex-wrap gap-2">
                  {PRESET_COLORS.map(color => (
                    <button
                      key={color}
                      type="button"
                      onClick={() => setFormData({ ...formData, color })}
                      className={clsx(
                        'w-8 h-8 rounded-full border-2 transition-all',
                        formData.color === color
                          ? 'border-gray-900 scale-110'
                          : 'border-gray-300 hover:border-gray-400'
                      )}
                      style={{ backgroundColor: color }}
                    />
                  ))}
                </div>
                <div className="mt-2">
                  <span
                    className="inline-flex items-center gap-2 px-3 py-1 rounded-full text-sm font-medium"
                    style={{
                      backgroundColor: `${formData.color}20`,
                      color: formData.color,
                      border: `1px solid ${formData.color}40`
                    }}
                  >
                    <TagIcon className="h-4 w-4" />
                    {formData.name || '预览'}
                  </span>
                </div>
              </div>

              <div className="flex gap-3">
                <Button type="submit" disabled={loading}>
                  {loading ? '保存中...' : (editingTag ? '更新' : '创建')}
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => {
                    setShowForm(false)
                    setEditingTag(undefined)
                  }}
                >
                  取消
                </Button>
              </div>
            </form>
          </div>
        )}

        {/* Tag List */}
        <div className="bg-white rounded-lg shadow-sm border border-gray-200">
          <div className="p-4 border-b">
            <h2 className="text-lg font-medium">所有标签</h2>
          </div>
          <div className="divide-y">
            {tags.length === 0 ? (
              <div className="p-8 text-center text-gray-500">
                还没有创建任何标签
              </div>
            ) : (
              tags.map(tag => (
                <div key={tag.id} className="p-4 flex items-center justify-between">
                  <div className="flex items-center gap-4">
                    <TagComponent tag={tag} />
                    <span className="text-sm text-gray-500">
                      {tag.ticket_count} 个 tickets
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleEdit(tag)}
                    >
                      <Edit2 className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleDelete(tag.id)}
                      className="text-red-600 hover:text-red-700"
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
```

#### 6.4 更新Ticket表单以支持标签（第6周周五）

**更新 TicketForm.tsx：**
```tsx
// 在表单中添加标签选择器
import { TagSelector } from './TagSelector'

// 在表单内容中添加：
<div>
  <label className="block text-sm font-medium text-gray-700 mb-2">
    标签
  </label>
  <TagSelector
    selectedTagIds={tagIds}
    onChange={setTagIds}
  />
</div>
```

#### 6.5 阶段六验收标准
- [ ] 标签的CRUD功能正常工作
- [ ] Ticket可以关联多个标签
- [ ] 标签选择器组件正常工作
- [ ] 标签管理页面功能完整
- [ ] 标签颜色系统正常显示

---

### 阶段七：搜索优化与部署（第7周）

#### 7.1 高级搜索实现（第7周周一、周二）

**后端搜索优化：**
```rust
// repositories/tickets.rs
impl TicketRepository {
    pub async fn search(
        &self,
        query: &str,
        tag_ids: Option<Vec<i64>>,
        status: Option<String>,
        page: u32,
        limit: u32,
    ) -> anyhow::Result<(Vec<Ticket>, i64)> {
        let offset = (page - 1) * limit;

        let mut sql = "
            SELECT DISTINCT t.* FROM tickets t
            LEFT JOIN ticket_tags tt ON t.id = tt.ticket_id
            WHERE 1=1
        ".to_string();

        let mut params = Vec::new();
        let mut param_count = 0;

        // 添加搜索条件
        if !query.is_empty() {
            param_count += 1;
            sql.push_str(&format!(" AND (to_tsvector('simple', t.title) @@ to_tsquery('simple', ${}) OR t.title ILIKE ${})", param_count, param_count + 1));
            params.push(format!("{}:*", query.replace(' ', " & ")));
            params.push(format!("%{}%", query));
            param_count += 1;
        }

        if let Some(status) = status {
            param_count += 1;
            sql.push_str(&format!(" AND t.status = ${}", param_count));
            params.push(status);
        }

        if let Some(tag_ids) = tag_ids {
            if !tag_ids.is_empty() {
                param_count += 1;
                let placeholders = tag_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                sql.push_str(&format!(" AND tt.tag_id IN (${})", param_count));
                params.extend(tag_ids.iter().map(|id| id.to_string()));
            }
        }

        sql.push_str(" ORDER BY t.created_at DESC");

        // 执行查询...
    }
}
```

#### 7.2 性能优化（第7周周三）

**前端优化：**
```tsx
// 使用 React.memo 优化组件
export const TicketCard = React.memo(function TicketCard({ ... }) {
  // 组件实现
})

// 使用 useMemo 优化计算
const filteredTickets = useMemo(() => {
  return tickets.filter(ticket =>
    ticket.title.toLowerCase().includes(searchTerm.toLowerCase())
  )
}, [tickets, searchTerm])
```

#### 7.3 测试实现（第7周周四）

**API测试：**
```rust
// tests/api_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_create_ticket() {
        // 测试创建ticket
    }

    #[tokio::test]
    async fn test_list_tickets() {
        // 测试获取ticket列表
    }
}
```

**前端测试：**
```typescript
// src/components/__tests__/TicketCard.test.tsx
import { render, screen } from '@testing-library/react'
import { TicketCard } from '../TicketCard'

describe('TicketCard', () => {
  it('renders ticket title', () => {
    const ticket = {
      id: 1,
      title: 'Test Ticket',
      description: 'Test Description',
      status: 'open',
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z',
    }

    render(<TicketCard
      ticket={ticket}
      onEdit={() => {}}
      onDelete={() => {}}
      onToggleStatus={() => {}}
    />)

    expect(screen.getByText('Test Ticket')).toBeInTheDocument()
  })
})
```

#### 7.4 部署准备（第7周周五）

**Docker配置：**
```dockerfile
# Dockerfile.backend
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ticket-backend /usr/local/bin/
EXPOSE 8080
CMD ["ticket-backend"]
```

```dockerfile
# Dockerfile.frontend
FROM node:20-alpine as builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
```

**docker-compose.yml：**
```yaml
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: ticket_db
      POSTGRES_USER: ticket_user
      POSTGRES_PASSWORD: secure_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  backend:
    build:
      context: ./ticket-backend
      dockerfile: Dockerfile
    environment:
      DATABASE_URL: postgresql://ticket_user:secure_password@postgres:5432/ticket_db
    depends_on:
      - postgres
    ports:
      - "8080:8080"

  frontend:
    build:
      context: ./ticket-frontend
      dockerfile: Dockerfile
    ports:
      - "80:80"
    depends_on:
      - backend

volumes:
  postgres_data:
```

#### 7.5 阶段七验收标准
- [ ] 搜索性能优化完成
- [ ] 组件性能优化完成
- [ ] 基础测试用例通过
- [ ] Docker配置完成
- [ ] 可以通过docker-compose启动完整系统

---

## 3. 关键里程碑（更新版）

| 里程碑 | 时间节点 | 交付物 | 验收标准 |
|--------|----------|--------|----------|
| M1: 环境搭建完成 | 第1周末 | 开发环境就绪 | 所有工具安装，项目初始化 |
| M2: 后端基础架构 | 第2周末 | 后端框架完成 | 数据库迁移，服务层搭建 |
| M3: 后端API实现 | 第3周末 | 完整的API | 所有CRUD接口可用 |
| M4: 前端框架完成 | 第4周末 | 前端基础架构 | 组件库，API服务层 |
| M5: 前端核心功能 | 第5周末 | Ticket管理功能 | 完整的CRUD界面 |
| M6: 标签系统完成 | 第6周末 | 标签功能 | 标签管理和关联 |
| M7: 系统优化部署 | 第7周末 | 生产就绪 | 性能优化，测试通过 |

## 3. 关键里程碑

| 里程碑 | 时间节点 | 交付物 | 验收标准 |
|--------|----------|--------|----------|
| M1: 基础架构完成 | 第2周末 | 可运行的Ticket管理系统 | 基础CRUD功能正常 |
| M2: 标签系统集成 | 第3周末 | 完整的标签功能 | 标签管理和过滤正常 |
| M3: 搜索功能完成 | 第4周末 | 高级搜索和过滤 | 复合查询功能正常 |
| M4: 生产就绪 | 第5周末 | 完整的生产系统 | 通过所有测试并成功部署 |

## 4. 风险管理

### 4.1 技术风险
| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| Rust学习曲线陡峭 | 中 | 高 | 提前学习，使用成熟框架 |
| PostgreSQL性能问题 | 高 | 低 | 合理设计索引，进行性能测试 |
| 前端状态管理复杂 | 中 | 中 | 使用Zustand简化状态管理 |
| 部署环境问题 | 高 | 低 | 提前准备部署文档和脚本 |

### 4.2 进度风险
- **缓解措施**: 每周进行进度回顾，及时调整计划
- **应急方案**: 优先实现核心功能，非关键功能可延后

## 5. 资源需求

### 5.1 开发环境
- 开发机：8GB+ RAM，SSD硬盘
- 数据库服务器：PostgreSQL 15+
- 测试环境：独立的测试数据库

### 5.2 生产环境
- 服务器：2核CPU，4GB RAM（最小配置）
- 数据库：PostgreSQL 15+，建议SSD存储
- Web服务器：Nginx
- 备份存储：至少保留30天的备份

## 6. 后续优化计划

### 6.1 短期优化（1-2个月）
- 实现批量操作功能
- 添加导入/导出功能
- 优化移动端体验
- 添加深色模式支持

### 6.2 中期扩展（3-6个月）
- 用户认证系统
- Ticket优先级
- 截止日期管理
- 实时通知系统

### 6.3 长期规划（6个月+）
- 多看板支持
- 团队协作功能
- 报表和统计
- API开放平台

## 7. 成功标准

### 7.1 功能完整性
- [ ] 所有核心功能按规格实现
- [ ] API接口完整可用
- [ ] 前端界面友好易用

### 7.2 性能指标
- [ ] API响应时间 < 200ms（95%请求）
- [ ] 页面加载时间 < 2s
- [ ] 支持1000+ Ticket无性能问题

### 7.3 质量标准
- [ ] 代码测试覆盖率 > 80%
- [ ] 无严重安全漏洞
- [ ] 通过跨浏览器测试

### 7.4 可维护性
- [ ] 代码结构清晰
- [ ] 文档完整
- [ ] 易于部署和维护

---

## 附录

### A. 每日工作建议

**第1周示例：**
- 周一：环境搭建，数据库初始化
- 周二：后端项目结构，基础路由
- 周三：Ticket模型和数据库层
- 周四：Ticket CRUD API
- 周五：前端项目初始化，基础组件
- 周末：联调测试，问题修复

### B. 代码审查清单

**后端审查点：**
- [ ] 错误处理完整
- [ ] SQL注入防护
- [ ] 输入验证
- [ ] 日志记录
- [ ] 性能优化

**前端审查点：**
- [ ] TypeScript类型定义
- [ ] 组件可复用性
- [ ] 性能优化
- [ ] 无障碍访问
- [ ] 响应式设计

### C. 测试用例示例

**API测试用例：**
```yaml
创建Ticket:
  - 标题: "创建正常Ticket"
    请求: { title: "测试任务", description: "这是一个测试" }
    期望: { status: 201, body_contains: ["id", "title"] }

  - 标题: "创建空标题Ticket"
    请求: { title: "" }
    期望: { status: 400, error: "标题不能为空" }
```

---

*文档版本: v1.0*
*最后更新: 2025-12-07*