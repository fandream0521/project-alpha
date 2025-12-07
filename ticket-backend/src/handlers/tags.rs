use crate::{
    error::AppError,
    models::{CreateTagRequest, Tag, TagWithCount, UpdateTagRequest},
    repositories::tags::TagRepository,
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

// 获取所有标签
pub async fn list_tags(
    State(pool): State<PgPool>,
    Query(query): Query<ListTagsQuery>,
) -> Result<Json<ListTagsResponse>, AppError> {
    let repository = TagRepository::new(pool);

    if query.with_counts.unwrap_or(false) {
        let tags = repository.list_with_counts().await?;
        Ok(Json(ListTagsResponse::WithCounts(tags)))
    } else {
        let tags = repository.list().await?;
        Ok(Json(ListTagsResponse::Simple(tags)))
    }
}

#[derive(Debug, Deserialize)]
pub struct ListTagsQuery {
    pub with_counts: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum ListTagsResponse {
    Simple(Vec<Tag>),
    WithCounts(Vec<TagWithCount>),
}

// 获取单个标签
pub async fn get_tag(
    State(pool): State<PgPool>,
    Path(id): Path<UuidType>,
) -> Result<Json<Tag>, AppError> {
    let repository = TagRepository::new(pool);
    let tag = repository.get_by_id(id).await?;
    Ok(Json(tag))
}

// 创建标签
pub async fn create_tag(
    State(pool): State<PgPool>,
    Json(request): Json<CreateTagRequest>,
) -> Result<Json<Tag>, AppError> {
    // 验证请求数据
    request.validate()
        .map_err(|e| AppError::Validation(e))?;

    // 检查名称是否已存在
    let repository = TagRepository::new(pool.clone());
    if repository.name_exists(&request.name, None).await? {
        return Err(AppError::conflict("标签名称"));
    }

    let tag = repository.create(request).await?;
    Ok(Json(tag))
}

// 更新标签
pub async fn update_tag(
    State(pool): State<PgPool>,
    Path(id): Path<UuidType>,
    Json(request): Json<UpdateTagRequest>,
) -> Result<Json<Tag>, AppError> {
    // 验证请求数据
    request.validate()
        .map_err(|e| AppError::Validation(e))?;

    let repository = TagRepository::new(pool);
    let tag = repository.update(id, request).await?;
    Ok(Json(tag))
}

// 删除标签
pub async fn delete_tag(
    State(pool): State<PgPool>,
    Path(id): Path<UuidType>,
) -> Result<StatusCode, AppError> {
    let repository = TagRepository::new(pool);
    repository.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// 搜索标签
pub async fn search_tags(
    State(pool): State<PgPool>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<Tag>>, AppError> {
    let repository = TagRepository::new(pool);
    let tags = repository.search(&query.q).await?;
    Ok(Json(tags))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

// 获取热门标签
pub async fn get_popular_tags(
    State(pool): State<PgPool>,
    Query(query): Query<PopularTagsQuery>,
) -> Result<Json<Vec<TagWithCount>>, AppError> {
    let repository = TagRepository::new(pool);
    let tags = repository.get_popular(query.limit).await?;
    Ok(Json(tags))
}

#[derive(Debug, Deserialize)]
pub struct PopularTagsQuery {
    pub limit: Option<i64>,
}

// 获取工单的标签
pub async fn get_ticket_tags(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<UuidType>,
) -> Result<Json<Vec<Tag>>, AppError> {
    let repository = TagRepository::new(pool);
    let tags = repository.get_ticket_tags(ticket_id).await?;
    Ok(Json(tags))
}

// 为工单设置标签
pub async fn set_ticket_tags(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<UuidType>,
    Json(request): Json<SetTicketTagsRequest>,
) -> Result<Json<Vec<Tag>>, AppError> {
    // 验证所有标签ID都存在
    let tag_repository = TagRepository::new(pool.clone());
    for tag_id in &request.tag_ids {
        if tag_repository.get_by_id(*tag_id).await.is_err() {
            return Err(AppError::not_found("标签"));
        }
    }

    // 先删除现有关联
    sqlx::query!("DELETE FROM ticket_tags WHERE ticket_id = $1", ticket_id)
        .execute(&pool)
        .await?;

    // 添加新关联
    if !request.tag_ids.is_empty() {
        let mut query_builder = sqlx::QueryBuilder::new("INSERT INTO ticket_tags (ticket_id, tag_id) VALUES ");

        let mut first = true;
        for tag_id in &request.tag_ids {
            if !first {
                query_builder.push(", ");
            }
            query_builder.push("(");
            query_builder.push_bind(ticket_id);
            query_builder.push(", ");
            query_builder.push_bind(*tag_id);
            query_builder.push(")");
            first = false;
        }

        query_builder.push(" ON CONFLICT DO NOTHING");
        query_builder.build().execute(&pool).await?;
    }

    // 返回更新后的标签列表
    let updated_tags = tag_repository.get_ticket_tags(ticket_id).await?;
    Ok(Json(updated_tags))
}

#[derive(Debug, Deserialize)]
pub struct SetTicketTagsRequest {
    pub tag_ids: Vec<UuidType>,
}

// 为工单添加单个标签
pub async fn add_ticket_tag(
    State(pool): State<PgPool>,
    Path((ticket_id, tag_id)): Path<(UuidType, UuidType)>,
) -> Result<Json<Vec<Tag>>, AppError> {
    // 验证标签存在
    let tag_repository = TagRepository::new(pool.clone());
    tag_repository.get_by_id(tag_id).await?;

    // 添加关联
    sqlx::query!(
        "INSERT INTO ticket_tags (ticket_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        ticket_id,
        tag_id
    )
    .execute(&pool)
    .await?;

    // 返回更新后的标签列表
    let updated_tags = tag_repository.get_ticket_tags(ticket_id).await?;
    Ok(Json(updated_tags))
}

// 从工单中移除标签
pub async fn remove_ticket_tag(
    State(pool): State<PgPool>,
    Path((ticket_id, tag_id)): Path<(UuidType, UuidType)>,
) -> Result<Json<Vec<Tag>>, AppError> {
    // 删除关联
    sqlx::query!(
        "DELETE FROM ticket_tags WHERE ticket_id = $1 AND tag_id = $2",
        ticket_id,
        tag_id
    )
    .execute(&pool)
    .await?;

    // 返回更新后的标签列表
    let tag_repository = TagRepository::new(pool);
    let updated_tags = tag_repository.get_ticket_tags(ticket_id).await?;
    Ok(Json(updated_tags))
}

// 获取标签统计信息
pub async fn get_tag_stats(
    State(pool): State<PgPool>,
) -> Result<Json<TagStats>, AppError> {
    let total_tags: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM tags")
        .fetch_one(&pool)
        .await?;

    let total_usage: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM ticket_tags")
        .fetch_one(&pool)
        .await?;

    let unused_tags: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM tags t
        LEFT JOIN ticket_tags tt ON t.id = tt.tag_id
        WHERE tt.tag_id IS NULL
        "#
    )
    .fetch_one(&pool)
    .await?;

    let most_used_tags = sqlx::query_as!(
        TagWithCount,
        r#"
        SELECT
            t.id, t.name, t.color, t.created_at, t.updated_at,
            COUNT(tt.tag_id) as "ticket_count!"
        FROM tags t
        LEFT JOIN ticket_tags tt ON t.id = tt.tag_id
        GROUP BY t.id, t.name, t.color, t.created_at, t.updated_at
        ORDER BY COUNT(tt.tag_id) DESC
        LIMIT 5
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(TagStats {
        total_tags,
        total_usage,
        unused_tags,
        most_used_tags,
    }))
}

#[derive(Debug, serde::Serialize)]
pub struct TagStats {
    pub total_tags: i64,
    pub total_usage: i64,
    pub unused_tags: i64,
    pub most_used_tags: Vec<TagWithCount>,
}