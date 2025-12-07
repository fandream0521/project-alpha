use crate::{
    database::DbPool,
    error::AppError,
    models::{
        CreateTicketRequest, PaginatedResponse, Ticket, TicketQuery, TicketStatus,
        TicketWithTags, UpdateTicketRequest,
    },
};
use sqlx::{query, query_as, query_scalar, Row};
use uuid::Uuid as UuidType;

pub struct TicketRepository {
    pool: DbPool,
}

impl TicketRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // 创建工单
    pub async fn create(&self, request: CreateTicketRequest) -> Result<Ticket, AppError> {
        let id = UuidType::new_v4();
        let now = chrono::Utc::now();
        let priority = request.priority.unwrap_or_default();

        let ticket = sqlx::query!(
            r#"
            INSERT INTO tickets (
                id, title, description, status, priority,
                assignee_id, reporter_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            RETURNING id, title, description, status, priority, assignee_id, reporter_id,
            created_at, updated_at, resolved_at
            "#,
            id,
            request.title,
            request.description,
            TicketStatus::Open as TicketStatus,
            priority as crate::models::Priority,
            request.assignee_id,
            request.reporter_id,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        // 转换为Ticket结构
        let ticket = Ticket {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            status: ticket.status,
            priority: ticket.priority,
            assignee_id: ticket.assignee_id,
            reporter_id: ticket.reporter_id,
            created_at: ticket.created_at,
            updated_at: ticket.updated_at,
            resolved_at: ticket.resolved_at,
        };

        // 如果有标签，关联标签
        if let Some(tag_ids) = &request.tag_ids {
            println!("DEBUG: Creating ticket {} with {} tags: {:?}", ticket.id, tag_ids.len(), tag_ids);
            if !tag_ids.is_empty() {
                self.associate_tags(ticket.id, tag_ids).await?;
                println!("DEBUG: Tags associated successfully");
            }
        } else {
            println!("DEBUG: No tags provided for ticket {}", ticket.id);
        }

        Ok(ticket)
    }

    // 根据ID获取工单
    pub async fn get_by_id(&self, id: UuidType) -> Result<Ticket, AppError> {
        let ticket = sqlx::query!(
            r#"
            SELECT id, title, description, status, priority, assignee_id, reporter_id,
            created_at, updated_at, resolved_at
            FROM tickets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::not_found("工单"))?;

        let ticket = Ticket {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            status: ticket.status,
            priority: ticket.priority,
            assignee_id: ticket.assignee_id,
            reporter_id: ticket.reporter_id,
            created_at: ticket.created_at,
            updated_at: ticket.updated_at,
            resolved_at: ticket.resolved_at,
        };

        Ok(ticket)
    }

    // 获取带标签的工单
    pub async fn get_with_tags(&self, id: UuidType) -> Result<TicketWithTags, AppError> {
        let ticket = self.get_by_id(id).await?;

        let tags = sqlx::query_as!(
            crate::models::Tag,
            r#"
            SELECT t.id, t.name, t.color, t.created_at, t.updated_at
            FROM tags t
            INNER JOIN ticket_tags tt ON t.id = tt.tag_id
            WHERE tt.ticket_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(TicketWithTags { ticket, tags })
    }

    // 更新工单
    pub async fn update(&self, id: UuidType, request: UpdateTicketRequest) -> Result<Ticket, AppError> {
        let now = chrono::Utc::now();

        // 构建动态更新查询
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(title) = request.title {
            updates.push("title = $2");
            params.push(title);
        }

        if let Some(description) = request.description {
            updates.push("description = $3");
            params.push(description);
        }

        if let Some(status) = request.status {
            updates.push("status = $4");
            params.push(status);
        }

        if let Some(priority) = request.priority {
            updates.push("priority = $5");
            params.push(priority);
        }

        if let Some(assignee_id) = request.assignee_id {
            updates.push("assignee_id = $6");
            params.push(assignee_id);
        }

        if updates.is_empty() {
            return self.get_by_id(id).await;
        }

        updates.push("updated_at = $7");

        let sql = format!(
            "UPDATE tickets SET {} WHERE id = $1 RETURNING id, title, description, status, priority, assignee_id, reporter_id, created_at, updated_at, resolved_at",
            updates.join(", ")
        );

        let mut query = sqlx::query(&sql).bind(id);

        // 动态绑定参数
        for param in params {
            if let Some(title) = request.title {
                query = query.bind(title);
            }
            if let Some(description) = request.description {
                query = query.bind(description);
            }
            if let Some(status) = request.status {
                query = query.bind(status);
            }
            if let Some(priority) = request.priority {
                query = query.bind(priority);
            }
            if let Some(assignee_id) = request.assignee_id {
                query = query.bind(assignee_id);
            }
        }
        query = query.bind(now);

        let ticket = query.fetch_one(&self.pool).await?;

        let ticket = Ticket {
            id: ticket.get(0),
            title: ticket.get(1),
            description: ticket.get(2),
            status: ticket.get(3),
            priority: ticket.get(4),
            assignee_id: ticket.get(5),
            reporter_id: ticket.get(6),
            created_at: ticket.get(7),
            updated_at: ticket.get(8),
            resolved_at: ticket.get(9),
        };

        // 更新标签关联
        if let Some(tag_ids) = request.tag_ids {
            self.update_tags(id, tag_ids).await?;
        }

        Ok(ticket)
    }

    // 删除工单
    pub async fn delete(&self, id: UuidType) -> Result<(), AppError> {
        // 先删除标签关联
        query!("DELETE FROM ticket_tags WHERE ticket_id = $1", id)
            .execute(&self.pool)
            .await?;

        // 再删除评论
        query!("DELETE FROM comments WHERE ticket_id = $1", id)
            .execute(&self.pool)
            .await?;

        // 最后删除工单
        let result = query!("DELETE FROM tickets WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::not_found("工单"));
        }

        Ok(())
    }

    // 列出工单（支持分页和过滤）
    pub async fn list(&self, query_params: TicketQuery) -> Result<PaginatedResponse<Ticket>, AppError> {
        let limit = query_params.limit.unwrap_or(20).min(100);
        let offset = query_params.offset.unwrap_or(0);

        // 构建WHERE条件
        let mut where_conditions = Vec::new();
        let mut count_where_conditions = Vec::new();

        if let Some(status) = query_params.status {
            where_conditions.push(format!("status = '{}'", status));
            count_where_conditions.push(format!("status = '{}'", status));
        }

        if let Some(priority) = query_params.priority {
            where_conditions.push(format!("priority = '{}'", priority));
            count_where_conditions.push(format!("priority = '{}'", priority));
        }

        if let Some(assignee_id) = query_params.assignee_id {
            where_conditions.push(format!("assignee_id = '{}'", assignee_id));
            count_where_conditions.push(format!("assignee_id = '{}'", assignee_id));
        }

        if let Some(reporter_id) = query_params.reporter_id {
            where_conditions.push(format!("reporter_id = '{}'", reporter_id));
            count_where_conditions.push(format!("reporter_id = '{}'", reporter_id));
        }

        if let Some(search) = query_params.search {
            if !search.trim().is_empty() {
                let search_condition = format!(
                    "(title ILIKE '%{}%' OR description ILIKE '%{}%')",
                    search.trim(),
                    search.trim()
                );
                where_conditions.push(search_condition.clone());
                count_where_conditions.push(search_condition);
            }
        }

        if let Some(tag_id) = query_params.tag_id {
            where_conditions.push(format!(
                "id IN (SELECT ticket_id FROM ticket_tags WHERE tag_id = '{}')",
                tag_id
            ));
            count_where_conditions.push(format!(
                "id IN (SELECT ticket_id FROM ticket_tags WHERE tag_id = '{}')",
                tag_id
            ));
        }

        let where_clause = if where_conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_conditions.join(" AND "))
        };

        let count_where_clause = if count_where_conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", count_where_conditions.join(" AND "))
        };

        // 构建ORDER BY
        let sort_by = query_params.sort_by.unwrap_or_else(|| "created_at".to_string());
        let sort_order = query_params.sort_order.unwrap_or_else(|| "desc".to_string());
        let order_clause = format!("ORDER BY {} {}", sort_by, sort_order);

        // 查询总数
        let count_sql = format!(
            "SELECT COUNT(*) FROM tickets {}",
            count_where_clause
        );

        let total: i64 = sqlx::query_scalar(&count_sql)
            .fetch_one(&self.pool)
            .await?;

        // 查询数据
        let sql = format!(
            r#"
            SELECT id, title, description, status, priority, assignee_id, reporter_id,
            created_at, updated_at, resolved_at
            FROM tickets {}
            {} LIMIT {} OFFSET {}
            "#,
            where_clause, order_clause, limit, offset
        );

        let rows = sqlx::query(&sql)
            .fetch_all(&self.pool)
            .await?;

        let tickets: Vec<Ticket> = rows.into_iter().map(|row| {
            Ticket {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                status: row.get(3),
                priority: row.get(4),
                assignee_id: row.get(5),
                reporter_id: row.get(6),
                created_at: row.get(7),
                updated_at: row.get(8),
                resolved_at: row.get(9),
            }
        }).collect();

        Ok(PaginatedResponse::new(tickets, total, limit, offset))
    }

    // 关联标签
    async fn associate_tags(&self, ticket_id: UuidType, tag_ids: &[UuidType]) -> Result<(), AppError> {
        for tag_id in tag_ids {
            query!(
                "INSERT INTO ticket_tags (ticket_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                ticket_id,
                tag_id
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    // 更新标签关联
    async fn update_tags(&self, ticket_id: UuidType, tag_ids: Vec<UuidType>) -> Result<(), AppError> {
        // 先删除现有关联
        query!("DELETE FROM ticket_tags WHERE ticket_id = $1", ticket_id)
            .execute(&self.pool)
            .await?;

        // 添加新关联
        if !tag_ids.is_empty() {
            self.associate_tags(ticket_id, &tag_ids).await?;
        }

        Ok(())
    }

    // 搜索工单
    pub async fn search(&self, term: &str) -> Result<Vec<Ticket>, AppError> {
        let search_pattern = format!("%{}%", term.trim());

        let rows = sqlx::query(
            r#"
            SELECT id, title, description, status, priority, assignee_id, reporter_id,
            created_at, updated_at, resolved_at
            FROM tickets
            WHERE title ILIKE $1 OR description ILIKE $1
            ORDER BY created_at DESC
            LIMIT 50
            "#
        )
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        let tickets: Vec<Ticket> = rows.into_iter().map(|row| {
            Ticket {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                status: row.get(3),
                priority: row.get(4),
                assignee_id: row.get(5),
                reporter_id: row.get(6),
                created_at: row.get(7),
                updated_at: row.get(8),
                resolved_at: row.get(9),
            }
        }).collect();

        Ok(tickets)
    }
}