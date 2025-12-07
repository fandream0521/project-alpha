use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::Value;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{debug, error};
use uuid::Uuid;

// 直接导入数据库类型
use sqlx::{PgPool, Row};
use std::env;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv::dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 直接初始化数据库连接池
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            tracing::info!("数据库连接成功！");
            pool
        }
        Err(e) => {
            tracing::error!("数据库连接失败: {}", e);
            std::process::exit(1);
        }
    };

    // 构建路由
    let app = Router::new()
        // 健康检查
        .route("/", get(health_check_basic))
        .route("/health", get(health_check_basic))
        .route("/api/health", get(health_check_detailed))
        .route("/api/db/stats", get(database_stats))
        .route("/api/db/optimize", post(database_optimize))
        // 标签API
        .route("/api/v1/tags", get(list_tags))
        .route("/api/v1/tags", post(create_tag))
        .route("/api/v1/tags/:id", get(get_tag))
        .route("/api/v1/tags/:id", put(update_tag))
        .route("/api/v1/tags/:id", delete(delete_tag))
        // 工单API
        .route("/api/v1/tickets", get(list_tickets))
        .route("/api/v1/tickets", post(create_ticket))
        .route("/api/v1/tickets/:id", get(get_ticket))
        .route("/api/v1/tickets/:id", put(update_ticket))
        .route("/api/v1/tickets/:id", delete(delete_ticket))
        .layer(CorsLayer::permissive())
        .layer(Extension(pool));

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!("Starting server on {}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 基础健康检查
async fn health_check_basic() -> &'static str {
    "Ticket backend is running!"
}

// 详细健康检查
async fn health_check_detailed(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, StatusCode> {
    let db_healthy = sqlx::query("SELECT 1").execute(&pool).await.is_ok();

    let status = if db_healthy { "healthy" } else { "unhealthy" };

    Ok(Json(serde_json::json!({
        "status": status,
        "database": db_healthy,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// 数据库优化 - 创建索引
async fn database_optimize(Extension(pool): Extension<PgPool>) -> Result<Json<Value>, StatusCode> {
    // 创建基础索引
    let indexes = vec![
        "CREATE INDEX IF NOT EXISTS idx_tickets_created_at ON tickets(created_at DESC)",
        "CREATE INDEX IF NOT EXISTS idx_tickets_status ON tickets(status)",
        "CREATE INDEX IF NOT EXISTS idx_tickets_priority ON tickets(priority)",
        "CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name)",
        "CREATE INDEX IF NOT EXISTS idx_ticket_tags_ticket_id ON ticket_tags(ticket_id)",
        "CREATE INDEX IF NOT EXISTS idx_ticket_tags_tag_id ON ticket_tags(tag_id)",
        "CREATE INDEX IF NOT EXISTS idx_tickets_status_created_at ON tickets(status, created_at DESC)",
    ];

    let mut created_indexes = Vec::new();
    let mut errors = Vec::new();

    for index_sql in indexes {
        match sqlx::query(index_sql).execute(&pool).await {
            Ok(_) => {
                created_indexes.push(index_sql.to_string());
                tracing::info!("索引创建成功: {}", index_sql);
            }
            Err(e) => {
                let error_msg = format!("索引创建失败 {}: {}", index_sql, e);
                errors.push(error_msg.clone());
                tracing::error!("{}", error_msg);
            }
        }
    }

    // 更新表统计信息
    match sqlx::query("ANALYZE tickets").execute(&pool).await {
        Ok(_) => tracing::info!("tickets 表统计信息已更新"),
        Err(e) => tracing::error!("更新 tickets 表统计信息失败: {}", e),
    }

    match sqlx::query("ANALYZE tags").execute(&pool).await {
        Ok(_) => tracing::info!("tags 表统计信息已更新"),
        Err(e) => tracing::error!("更新 tags 表统计信息失败: {}", e),
    }

    Ok(Json(serde_json::json!({
        "message": "数据库优化完成",
        "created_indexes": created_indexes.len(),
        "errors": errors,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// 数据库统计信息
async fn database_stats(Extension(pool): Extension<PgPool>) -> Result<Json<Value>, StatusCode> {
    let tickets_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tickets")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let tags_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tags")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let comments_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comments")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "tickets_count": tickets_count,
        "tags_count": tags_count,
        "comments_count": comments_count
    })))
}

// 标签API
async fn list_tags(Extension(pool): Extension<PgPool>) -> Result<Json<Value>, StatusCode> {
    let rows =
        sqlx::query("SELECT id, name, color, created_at, updated_at FROM tags ORDER BY name")
            .fetch_all(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tags: Vec<Value> = rows
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<Uuid, _>(0),
                "name": row.get::<String, _>(1),
                "color": row.get::<String, _>(2),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(3),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(4),
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "data": tags,
        "total": tags.len()
    })))
}

async fn create_tag(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let name = request
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let color = request
        .get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#3B82F6");

    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let row = sqlx::query(
        "INSERT INTO tags (id, name, color, created_at, updated_at) VALUES ($1, $2, $3, $4, $4) RETURNING id, name, color, created_at, updated_at"
    )
    .bind(id)
    .bind(name)
    .bind(color)
    .bind(now)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": row.get::<Uuid, _>(0),
        "name": row.get::<String, _>(1),
        "color": row.get::<String, _>(2),
        "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(3),
        "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(4),
    })))
}

async fn get_tag(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let row = sqlx::query("SELECT id, name, color, created_at, updated_at FROM tags WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": row.get::<Uuid, _>(0),
        "name": row.get::<String, _>(1),
        "color": row.get::<String, _>(2),
        "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(3),
        "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(4),
    })))
}

async fn update_tag(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let name = request.get("name").and_then(|v| v.as_str());
    let color = request.get("color").and_then(|v| v.as_str());

    let now = chrono::Utc::now();

    if let (Some(name), Some(color)) = (name, color) {
        let row = sqlx::query(
            "UPDATE tags SET name = $1, color = $2, updated_at = $3 WHERE id = $4 RETURNING id, name, color, created_at, updated_at"
        )
        .bind(name)
        .bind(color)
        .bind(now)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(serde_json::json!({
            "id": row.get::<Uuid, _>(0),
            "name": row.get::<String, _>(1),
            "color": row.get::<String, _>(2),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(3),
            "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(4),
        })))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn delete_tag(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// 高级工单查询结构
#[derive(serde::Deserialize)]
struct TicketQuery {
    search: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    tag_ids: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
}

// 工单API - 支持高级搜索（优化版）
async fn list_tickets(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<TicketQuery>,
) -> Result<Json<Value>, StatusCode> {
    let result = async move {
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = (page - 1) * limit;

        // 构建动态SQL查询
        let mut sql = "
            SELECT DISTINCT t.id, t.title, t.description, t.status, t.priority,
                   t.assignee_id, t.reporter_id, t.created_at, t.updated_at, t.resolved_at
            FROM tickets t
            LEFT JOIN ticket_tags tt ON t.id = tt.ticket_id
            WHERE 1=1
        "
        .to_string();

        let mut conditions: Vec<String> = Vec::new();
        let mut params = Vec::new();

        // 添加搜索条件
        if let Some(search_term) = &query.search {
            if !search_term.trim().is_empty() {
                conditions.push("(t.title LIKE $1 OR t.description LIKE $1)".to_string());
                params.push(format!("%{}%", search_term.trim()));
            }
        }

        // 添加状态过滤
        if let Some(status) = &query.status {
            conditions.push("t.status = $".to_string() + &(params.len() + 1).to_string());
            params.push(status.clone());
        }

        // 添加优先级过滤
        if let Some(priority) = &query.priority {
            conditions.push("t.priority = $".to_string() + &(params.len() + 1).to_string());
            params.push(priority.clone());
        }

        // 添加标签过滤
        if let Some(tag_ids) = &query.tag_ids {
            if !tag_ids.trim().is_empty() {
                let tag_ids: Vec<&str> = tag_ids.split(',').collect();
                let mut placeholders = Vec::new();
                for tag_id in &tag_ids {
                    placeholders.push("$".to_string() + &(params.len() + 1).to_string());
                    params.push(tag_id.trim().to_string());
                }
                conditions.push("tt.tag_id IN (".to_string() + &placeholders.join(",") + ")");
            }
        }

        // 组合WHERE条件
        if !conditions.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&conditions.join(" AND "));
        }

        // 添加排序和分页
        sql.push_str(" ORDER BY t.created_at DESC LIMIT $");
        sql.push_str(&(params.len() + 1).to_string());
        sql.push_str(" OFFSET $");
        sql.push_str(&(params.len() + 2).to_string());

        // 执行查询
        let mut query_builder = sqlx::query(&sql);

        // 绑定字符串参数
        for param in &params {
            query_builder = query_builder.bind(param);
        }

        // 绑定整数参数 (limit 和 offset)
        query_builder = query_builder.bind(limit as i64);
        query_builder = query_builder.bind(offset as i64);

        let rows = match query_builder.fetch_all(&pool).await {
            Ok(rows) => rows,
            Err(e) => {
                error!("Error fetching tickets: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        // 获取所有ticket的ID，然后批量获取包含标签的完整数据
        let ticket_ids: Vec<Uuid> = rows.into_iter().map(|row| row.get::<Uuid, _>(0)).collect();

        // 如果没有找到tickets，直接返回空结果
        if ticket_ids.is_empty() {
            debug!("No tickets found, returning empty result");
            return Ok(serde_json::json!({
                "data": [],
                "total": 0,
                "page": page,
                "limit": limit
            }));
        }

        // 构建查询获取tickets和它们的标签
        let tickets_with_tags_sql = format!(
            "SELECT
                t.id, t.title, t.description, t.status, t.priority,
                t.assignee_id, t.reporter_id, t.created_at, t.updated_at, t.resolved_at,
                COALESCE(
                    JSON_AGG(
                        JSON_BUILD_OBJECT(
                            'id', tg.id,
                            'name', tg.name,
                            'color', tg.color,
                            'created_at', tg.created_at,
                            'updated_at', tg.updated_at
                        )
                    ) FILTER (WHERE tg.id IS NOT NULL),
                    '[]'::json
                ) as tags
             FROM tickets t
             LEFT JOIN ticket_tags tt ON t.id = tt.ticket_id
             LEFT JOIN tags tg ON tt.tag_id = tg.id
             WHERE t.id = ANY($1)
             GROUP BY t.id, t.title, t.description, t.status, t.priority,
                      t.assignee_id, t.reporter_id, t.created_at, t.updated_at, t.resolved_at
             ORDER BY t.created_at DESC"
        );

        let tickets_with_tags = match sqlx::query(&tickets_with_tags_sql)
            .bind(&ticket_ids)
            .fetch_all(&pool)
            .await
        {
            Ok(tickets) => tickets,
            Err(e) => {
                error!("Error fetching tickets with tags: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let tickets: Vec<Value> = tickets_with_tags
            .into_iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<Uuid, _>(0),
                    "title": row.get::<String, _>(1),
                    "description": row.get::<Option<String>, _>(2),
                    "status": row.get::<String, _>(3),
                    "priority": row.get::<String, _>(4),
                    "assignee_id": row.get::<Option<Uuid>, _>(5),
                    "reporter_id": row.get::<Option<Uuid>, _>(6),
                    "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(7),
                    "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(8),
                    "resolved_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>(9),
                    "tags": row.get::<serde_json::Value, _>(10), // tags as JSON
                })
            })
            .collect();

        // 获取总数（用于分页）
        let count_sql = "
            SELECT COUNT(DISTINCT t.id) as total
            FROM tickets t
            LEFT JOIN ticket_tags tt ON t.id = tt.ticket_id
            WHERE 1=1
        "
        .to_string();

        let mut count_conditions: Vec<String> = Vec::new();
        let mut count_params = Vec::new();

        // 为计数查询重新构建条件
        if let Some(search_term) = &query.search {
            if !search_term.trim().is_empty() {
                count_conditions.push("(t.title LIKE $1 OR t.description LIKE $1)".to_string());
                count_params.push(format!("%{}%", search_term.trim()));
            }
        }

        if let Some(status) = &query.status {
            count_conditions
                .push("t.status = $".to_string() + &(count_params.len() + 1).to_string());
            count_params.push(status.clone());
        }

        if let Some(priority) = &query.priority {
            count_conditions
                .push("t.priority = $".to_string() + &(count_params.len() + 1).to_string());
            count_params.push(priority.clone());
        }

        if let Some(tag_ids) = &query.tag_ids {
            if !tag_ids.trim().is_empty() {
                let tag_ids: Vec<&str> = tag_ids.split(',').collect();
                let placeholders: Vec<String> = tag_ids
                    .iter()
                    .map(|_| "$".to_string() + &(count_params.len() + 1).to_string())
                    .collect();
                count_conditions.push("tt.tag_id IN (".to_string() + &placeholders.join(",") + ")");
                for tag_id in tag_ids {
                    count_params.push(tag_id.trim().to_string());
                }
            }
        }

        let mut final_count_sql = count_sql;
        if !count_conditions.is_empty() {
            final_count_sql.push_str(" AND ");
            final_count_sql.push_str(&count_conditions.join(" AND "));
        }

        let mut count_query_builder = sqlx::query(&final_count_sql);
        for param in &count_params {
            count_query_builder = count_query_builder.bind(param);
        }

        let total: i64 = count_query_builder
            .fetch_optional(&pool)
            .await
            .map(|row| row.map(|r| r.get(0)).unwrap_or(0))
            .unwrap_or(0);

        Ok(serde_json::json!({
            "data": tickets,
            "total": total,
            "page": page,
            "limit": limit
        }))
    }
    .await;

    match result {
        Ok(json_value) => Ok(Json(json_value)),
        Err(e) => {
            error!("Error in list_tickets: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_ticket(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let title = request
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let description = request.get("description").and_then(|v| v.as_str());
    let priority = request
        .get("priority")
        .and_then(|v| v.as_str())
        .unwrap_or("medium");

    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let row = sqlx::query(
        "INSERT INTO tickets (id, title, description, status, priority, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $6) RETURNING id, title, description, status, priority, assignee_id, reporter_id, created_at, updated_at, resolved_at"
    )
    .bind(id)
    .bind(title)
    .bind(description)
    .bind("open")
    .bind(priority)
    .bind(now)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": row.get::<Uuid, _>(0),
        "title": row.get::<String, _>(1),
        "description": row.get::<Option<String>, _>(2),
        "status": row.get::<String, _>(3),
        "priority": row.get::<String, _>(4),
        "assignee_id": row.get::<Option<Uuid>, _>(5),
        "reporter_id": row.get::<Option<Uuid>, _>(6),
        "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(7),
        "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(8),
        "resolved_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>(9),
    })))
}

async fn get_ticket(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, title, description, status, priority, assignee_id, reporter_id, created_at, updated_at, resolved_at FROM tickets WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": row.get::<Uuid, _>(0),
        "title": row.get::<String, _>(1),
        "description": row.get::<Option<String>, _>(2),
        "status": row.get::<String, _>(3),
        "priority": row.get::<String, _>(4),
        "assignee_id": row.get::<Option<Uuid>, _>(5),
        "reporter_id": row.get::<Option<Uuid>, _>(6),
        "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(7),
        "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(8),
        "resolved_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>(9),
    })))
}

async fn update_ticket(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // 将字符串ID解析为UUID
    let ticket_id = id
        .parse::<uuid::Uuid>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let title = request.get("title").and_then(|v| v.as_str());
    let description = request.get("description").and_then(|v| v.as_str());
    let status = request.get("status").and_then(|v| v.as_str());
    let priority = request.get("priority").and_then(|v| v.as_str());

    // 验证状态值
    if let Some(status_val) = status {
        let valid_statuses = ["open", "in_progress", "resolved", "closed"];
        if !valid_statuses.contains(&status_val) {
            error!("Invalid status value: {}", status_val);
            return Err(StatusCode::BAD_REQUEST);
        }
        debug!("Valid status value: {}", status_val);
    }

    // 验证优先级值
    if let Some(priority_val) = priority {
        let valid_priorities = ["low", "medium", "high", "urgent"];
        if !valid_priorities.contains(&priority_val) {
            error!("Invalid priority value: {}", priority_val);
            return Err(StatusCode::BAD_REQUEST);
        }
        debug!("Valid priority value: {}", priority_val);
    }

    let now = chrono::Utc::now();

    // 使用基础更新，更新所有非空字段
    let row_result = sqlx::query(
        "UPDATE tickets SET
         title = COALESCE($1, title),
         description = COALESCE($2, description),
         status = COALESCE($3, status),
         priority = COALESCE($4, priority),
         updated_at = $5
         WHERE id = $6
         RETURNING id, title, description, status, priority, assignee_id, reporter_id, created_at, updated_at, resolved_at"
    )
    .bind(title)
    .bind(description)
    .bind(status)
    .bind(priority)
    .bind(now)
    .bind(ticket_id)
    .fetch_optional(&pool)
    .await;

    let row = match row_result {
        Ok(Some(row)) => row,
        Ok(None) => {
            debug!("No ticket found with id: {}", id);
            return Err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            error!("Database error updating ticket {}: {}", id, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(serde_json::json!({
        "id": row.get::<Uuid, _>(0),
        "title": row.get::<String, _>(1),
        "description": row.get::<Option<String>, _>(2),
        "status": row.get::<String, _>(3),
        "priority": row.get::<String, _>(4),
        "assignee_id": row.get::<Option<Uuid>, _>(5),
        "reporter_id": row.get::<Option<Uuid>, _>(6),
        "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(7),
        "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>(8),
        "resolved_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>(9),
    })))
}

async fn delete_ticket(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM tickets WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
