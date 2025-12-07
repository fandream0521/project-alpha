use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// 工单状态枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Default for TicketStatus {
    fn default() -> Self {
        Self::Open
    }
}

// 优先级枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Medium
    }
}

// 标签模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 创建标签请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTagRequest {
    #[validate(length(min = 1, max = 50, message = "标签名称长度必须在1-50个字符之间"))]
    pub name: String,
    #[validate(length(min = 7, max = 7, message = "颜色必须是7位HEX值"))]
    pub color: Option<String>,
}

// 更新标签请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTagRequest {
    #[validate(length(min = 1, max = 50, message = "标签名称长度必须在1-50个字符之间"))]
    pub name: Option<String>,
    #[validate(length(min = 7, max = 7, message = "颜色必须是7位HEX值"))]
    pub color: Option<String>,
}

// 工单模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: Priority,
    pub assignee_id: Option<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

// 创建工单请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTicketRequest {
    #[validate(length(min = 1, max = 255, message = "标题长度必须在1-255个字符之间"))]
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>, // 标签ID列表
}

// 更新工单请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTicketRequest {
    #[validate(length(min = 1, max = 255, message = "标题长度必须在1-255个字符之间"))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>, // 标签ID列表
}

// 工单查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct TicketQuery {
    pub status: Option<TicketStatus>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub search: Option<String>, // 搜索关键词
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<String>,    // 排序字段
    pub sort_order: Option<String>, // 排序方向 asc/desc
}

// 评论模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub author_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 创建评论请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, message = "评论内容不能为空"))]
    pub content: String,
}

// 更新评论请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1, message = "评论内容不能为空"))]
    pub content: String,
}

// 带标签的工单模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketWithTags {
    #[serde(flatten)]
    pub ticket: Ticket,
    pub tags: Vec<Tag>,
}

// 带计数的标签模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagWithCount {
    #[serde(flatten)]
    pub tag: Tag,
    pub ticket_count: i64,
}

// 带评论的工单模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketWithDetails {
    #[serde(flatten)]
    pub ticket: Ticket,
    pub tags: Vec<Tag>,
    pub comments: Vec<Comment>,
}

// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, limit: i64, offset: i64) -> Self {
        Self {
            data,
            total,
            limit,
            offset,
        }
    }
}
