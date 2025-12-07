use crate::{
    error::AppError,
    models::{CreateTicketRequest, Ticket, TicketQuery, TicketWithDetails, UpdateTicketRequest},
    repositories::tickets::TicketRepository,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid as UuidType;
use validator::Validate;

// 获取工单列表查询参数
#[derive(Debug, Deserialize)]
pub struct ListTicketsQuery {
    pub status: Option<crate::models::TicketStatus>,
    pub priority: Option<crate::models::Priority>,
    pub assignee_id: Option<UuidType>,
    pub reporter_id: Option<UuidType>,
    pub tag_id: Option<UuidType>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

// 获取工单列表
pub async fn list_tickets(
    State(pool): State<PgPool>,
    Query(query): Query<ListTicketsQuery>,
) -> Result<Json<crate::models::PaginatedResponse<Ticket>>, AppError> {
    let repository = TicketRepository::new(pool);
    let ticket_query = TicketQuery {
        status: query.status,
        priority: query.priority,
        assignee_id: query.assignee_id,
        reporter_id: query.reporter_id,
        tag_id: query.tag_id,
        search: query.search,
        limit: query.limit,
        offset: query.offset,
        sort_by: query.sort_by,
        sort_order: query.sort_order,
    };

    let result = repository.list(ticket_query).await?;
    Ok(Json(result))
}

// 获取单个工单详情（带标签和评论）
pub async fn get_ticket(
    State(pool): State<PgPool>,
    Path(id): Path<UuidType>,
) -> Result<Json<TicketWithDetails>, AppError> {
    let ticket_repository = TicketRepository::new(pool.clone());

    // 获取带标签的工单
    let ticket_with_tags = ticket_repository.get_with_tags(id).await?;

    // 获取评论
    let comments = sqlx::query_as!(
        crate::models::Comment,
        r#"
        SELECT id, ticket_id, author_id, content, created_at, updated_at
        FROM comments
        WHERE ticket_id = $1
        ORDER BY created_at ASC
        "#,
        id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(TicketWithDetails {
        ticket: ticket_with_tags.ticket,
        tags: ticket_with_tags.tags,
        comments,
    }))
}

// 创建工单
pub async fn create_ticket(
    State(pool): State<PgPool>,
    Json(request): Json<CreateTicketRequest>,
) -> Result<Json<Ticket>, AppError> {
    // 验证请求数据
    request.validate()
        .map_err(|e| AppError::Validation(e))?;

    let repository = TicketRepository::new(pool);
    let ticket = repository.create(request).await?;

    Ok(Json(ticket))
}

// 更新工单
pub async fn update_ticket(
    State(pool): State<PgPool>,
    Path(id): Path<UuidType>,
    Json(request): Json<UpdateTicketRequest>,
) -> Result<Json<Ticket>, AppError> {
    // 验证请求数据
    request.validate()
        .map_err(|e| AppError::Validation(e))?;

    let repository = TicketRepository::new(pool);
    let ticket = repository.update(id, request).await?;

    Ok(Json(ticket))
}

// 删除工单
pub async fn delete_ticket(
    State(pool): State<PgPool>,
    Path(id): Path<UuidType>,
) -> Result<StatusCode, AppError> {
    let repository = TicketRepository::new(pool);
    repository.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// 搜索工单
pub async fn search_tickets(
    State(pool): State<PgPool>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<Ticket>>, AppError> {
    let repository = TicketRepository::new(pool);
    let tickets = repository.search(&query.q).await?;
    Ok(Json(tickets))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

// 批量更新工单状态
pub async fn bulk_update_status(
    State(pool): State<PgPool>,
    Json(request): Json<BulkUpdateStatusRequest>,
) -> Result<Json<BulkUpdateResult>, AppError> {
    let repository = TicketRepository::new(pool);
    let mut updated_count = 0;
    let mut errors = Vec::new();

    for ticket_id in request.ticket_ids {
        let update_request = UpdateTicketRequest {
            title: None,
            description: None,
            status: Some(request.status.clone()),
            priority: None,
            assignee_id: None,
            tag_ids: None,
        };

        match repository.update(ticket_id, update_request).await {
            Ok(_) => updated_count += 1,
            Err(e) => errors.push(format!("{}: {}", ticket_id, e)),
        }
    }

    Ok(Json(BulkUpdateResult {
        updated_count,
        total_count: request.ticket_ids.len(),
        errors: if errors.is_empty() { None } else { Some(errors) },
    }))
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateStatusRequest {
    pub ticket_ids: Vec<UuidType>,
    pub status: crate::models::TicketStatus,
}

#[derive(Debug, serde::Serialize)]
pub struct BulkUpdateResult {
    pub updated_count: usize,
    pub total_count: usize,
    pub errors: Option<Vec<String>>,
}

// 添加评论到工单
pub async fn add_comment(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<UuidType>,
    Json(request): Json<crate::models::CreateCommentRequest>,
) -> Result<Json<crate::models::Comment>, AppError> {
    // 验证请求数据
    request.validate()
        .map_err(|e| AppError::Validation(e))?;

    // 验证工单是否存在
    let ticket_repository = TicketRepository::new(pool.clone());
    if ticket_repository.get_by_id(ticket_id).await.is_err() {
        return Err(AppError::not_found("工单"));
    }

    let comment_id = UuidType::new_v4();
    let now = chrono::Utc::now();

    let comment = sqlx::query_as!(
        crate::models::Comment,
        r#"
        INSERT INTO comments (id, ticket_id, author_id, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $5)
        RETURNING id, ticket_id, author_id, content, created_at, updated_at
        "#,
        comment_id,
        ticket_id,
        request.author_id,
        request.content,
        now
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(comment))
}

// 获取工单统计信息
pub async fn get_ticket_stats(
    State(pool): State<PgPool>,
    Query(query): Query<TicketStatsQuery>,
) -> Result<Json<TicketStats>, AppError> {
    let mut where_conditions = Vec::new();

    if let Some(assignee_id) = query.assignee_id {
        where_conditions.push(format!("assignee_id = '{}'", assignee_id));
    }

    let where_clause = if where_conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_conditions.join(" AND "))
    };

    // 获取各状态的数量
    let status_counts = sqlx::query!(
        r#"
        SELECT status, COUNT(*) as count
        FROM tickets
        {}
        GROUP BY status
        "#,
        where_clause
    )
    .fetch_all(&pool)
    .await?;

    // 获取各优先级的数量
    let priority_counts = sqlx::query!(
        r#"
        SELECT priority, COUNT(*) as count
        FROM tickets
        {}
        GROUP BY priority
        "#,
        where_clause
    )
    .fetch_all(&pool)
    .await?;

    let total_open = status_counts
        .iter()
        .filter(|r| matches!(r.status, crate::models::TicketStatus::Open))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    let total_in_progress = status_counts
        .iter()
        .filter(|r| matches!(r.status, crate::models::TicketStatus::InProgress))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    let total_resolved = status_counts
        .iter()
        .filter(|r| matches!(r.status, crate::models::TicketStatus::Resolved))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    let total_closed = status_counts
        .iter()
        .filter(|r| matches!(r.status, crate::models::TicketStatus::Closed))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    let total_urgent = priority_counts
        .iter()
        .filter(|r| matches!(r.priority, crate::models::Priority::Urgent))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    let total_high = priority_counts
        .iter()
        .filter(|r| matches!(r.priority, crate::models::Priority::High))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    let total_medium = priority_counts
        .iter()
        .filter(|r| matches!(r.priority, crate::models::Priority::Medium))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    let total_low = priority_counts
        .iter()
        .filter(|r| matches!(r.priority, crate::models::Priority::Low))
        .map(|r| r.count.unwrap_or(0))
        .sum::<i64>();

    Ok(Json(TicketStats {
        total_by_status: StatusStats {
            open: total_open,
            in_progress: total_in_progress,
            resolved: total_resolved,
            closed: total_closed,
        },
        total_by_priority: PriorityStats {
            urgent: total_urgent,
            high: total_high,
            medium: total_medium,
            low: total_low,
        },
    }))
}

#[derive(Debug, Deserialize)]
pub struct TicketStatsQuery {
    pub assignee_id: Option<UuidType>,
}

#[derive(Debug, serde::Serialize)]
pub struct TicketStats {
    pub total_by_status: StatusStats,
    pub total_by_priority: PriorityStats,
}

#[derive(Debug, serde::Serialize)]
pub struct StatusStats {
    pub open: i64,
    pub in_progress: i64,
    pub resolved: i64,
    pub closed: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct PriorityStats {
    pub urgent: i64,
    pub high: i64,
    pub medium: i64,
    pub low: i64,
}