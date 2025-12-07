use crate::{
    database::DbPool,
    error::AppError,
    models::{CreateTagRequest, Tag, TagWithCount, UpdateTagRequest},
};
use sqlx::{query, query_as, query_scalar, Row};
use uuid::Uuid as UuidType;

pub struct TagRepository {
    pool: DbPool,
}

impl TagRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // 创建标签
    pub async fn create(&self, request: CreateTagRequest) -> Result<Tag, AppError> {
        let id = UuidType::new_v4();
        let now = chrono::Utc::now();
        let color = request.color.unwrap_or_else(|| "#3B82F6".to_string());

        let tag = sqlx::query!(
            r#"
            INSERT INTO tags (id, name, color, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id, name, color, created_at, updated_at
            "#,
            id,
            request.name,
            color,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        let tag = Tag {
            id: tag.id,
            name: tag.name,
            color: tag.color,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
        };

        Ok(tag)
    }

    // 根据ID获取标签
    pub async fn get_by_id(&self, id: UuidType) -> Result<Tag, AppError> {
        let tag = sqlx::query!(
            "SELECT id, name, color, created_at, updated_at FROM tags WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::not_found("标签"))?;

        let tag = Tag {
            id: tag.id,
            name: tag.name,
            color: tag.color,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
        };

        Ok(tag)
    }

    // 根据名称获取标签
    pub async fn get_by_name(&self, name: &str) -> Result<Tag, AppError> {
        let tag = sqlx::query!(
            "SELECT id, name, color, created_at, updated_at FROM tags WHERE name = $1",
            name
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::not_found("标签"))?;

        let tag = Tag {
            id: tag.id,
            name: tag.name,
            color: tag.color,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
        };

        Ok(tag)
    }

    // 检查名称是否已存在
    pub async fn name_exists(&self, name: &str, exclude_id: Option<UuidType>) -> Result<bool, AppError> {
        let count: i64 = if let Some(exclude_id) = exclude_id {
            query_scalar!(
                "SELECT COUNT(*) FROM tags WHERE name = $1 AND id != $2",
                name,
                exclude_id
            )
            .fetch_one(&self.pool)
            .await?
        } else {
            query_scalar!("SELECT COUNT(*) FROM tags WHERE name = $1", name)
                .fetch_one(&self.pool)
                .await?
        };

        Ok(count > 0)
    }

    // 获取所有标签
    pub async fn list(&self) -> Result<Vec<Tag>, AppError> {
        let rows = sqlx::query!(
            "SELECT id, name, color, created_at, updated_at FROM tags ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        let tags: Vec<Tag> = rows.into_iter().map(|row| {
            Tag {
                id: row.id,
                name: row.name,
                color: row.color,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(tags)
    }

    // 获取带工单数量的标签
    pub async fn list_with_counts(&self) -> Result<Vec<TagWithCount>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                t.id, t.name, t.color, t.created_at, t.updated_at,
                COALESCE(tt.ticket_count, 0) as "ticket_count!"
            FROM tags t
            LEFT JOIN (
                SELECT tag_id, COUNT(*) as ticket_count
                FROM ticket_tags
                GROUP BY tag_id
            ) tt ON t.id = tt.tag_id
            ORDER BY t.name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let tags: Vec<TagWithCount> = rows.into_iter().map(|row| {
            let tag = Tag {
                id: row.id,
                name: row.name,
                color: row.color,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };

            TagWithCount {
                tag,
                ticket_count: row.ticket_count,
            }
        }).collect();

        Ok(tags)
    }

    // 更新标签
    pub async fn update(&self, id: UuidType, request: UpdateTagRequest) -> Result<Tag, AppError> {
        let now = chrono::Utc::now();

        // 检查名称是否已存在
        if let Some(ref name) = request.name {
            if self.name_exists(name, Some(id)).await? {
                return Err(AppError::conflict("标签名称"));
            }
        }

        // 构建动态更新查询
        let mut updates = Vec::new();
        let mut query = sqlx::query("UPDATE tags SET updated_at = $1").bind(now);

        if let Some(name) = request.name {
            updates.push(", name = $2");
            query = query.bind(name);
        }

        if let Some(color) = request.color {
            updates.push(", color = $3");
            query = query.bind(color);
        }

        let sql = format!("{} WHERE id = $4 RETURNING id, name, color, created_at, updated_at",
                         format!("UPDATE tags SET updated_at = $1{}", updates.join("")));

        let mut query = sqlx::query(&sql).bind(now);

        if let Some(name) = request.name {
            query = query.bind(name);
        }
        if let Some(color) = request.color {
            query = query.bind(color);
        }
        query = query.bind(id);

        let tag = query.fetch_one(&self.pool).await?;

        let tag = Tag {
            id: tag.get(0),
            name: tag.get(1),
            color: tag.get(2),
            created_at: tag.get(3),
            updated_at: tag.get(4),
        };

        Ok(tag)
    }

    // 删除标签
    pub async fn delete(&self, id: UuidType) -> Result<(), AppError> {
        // 先删除与工单的关联
        query!("DELETE FROM ticket_tags WHERE tag_id = $1", id)
            .execute(&self.pool)
            .await?;

        // 删除标签
        let result = query!("DELETE FROM tags WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::not_found("标签"));
        }

        Ok(())
    }

    // 获取工单的标签
    pub async fn get_ticket_tags(&self, ticket_id: UuidType) -> Result<Vec<Tag>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT t.id, t.name, t.color, t.created_at, t.updated_at
            FROM tags t
            INNER JOIN ticket_tags tt ON t.id = tt.tag_id
            WHERE tt.ticket_id = $1
            ORDER BY t.name
            "#,
            ticket_id
        )
        .fetch_all(&self.pool)
        .await?;

        let tags: Vec<Tag> = rows.into_iter().map(|row| {
            Tag {
                id: row.id,
                name: row.name,
                color: row.color,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(tags)
    }

    // 搜索标签
    pub async fn search(&self, term: &str) -> Result<Vec<Tag>, AppError> {
        let search_pattern = format!("%{}%", term.trim());

        let rows = sqlx::query!(
            r#"
            SELECT id, name, color, created_at, updated_at
            FROM tags
            WHERE name ILIKE $1
            ORDER BY name
            LIMIT 20
            "#,
            search_pattern
        )
        .fetch_all(&self.pool)
        .await?;

        let tags: Vec<Tag> = rows.into_iter().map(|row| {
            Tag {
                id: row.id,
                name: row.name,
                color: row.color,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(tags)
    }

    // 获取热门标签（按使用次数排序）
    pub async fn get_popular(&self, limit: Option<i64>) -> Result<Vec<TagWithCount>, AppError> {
        let limit = limit.unwrap_or(10);

        let rows = sqlx::query!(
            r#"
            SELECT
                t.id, t.name, t.color, t.created_at, t.updated_at,
                COALESCE(tt.ticket_count, 0) as "ticket_count!"
            FROM tags t
            LEFT JOIN (
                SELECT tag_id, COUNT(*) as ticket_count
                FROM ticket_tags
                GROUP BY tag_id
            ) tt ON t.id = tt.tag_id
            WHERE tt.ticket_count > 0
            ORDER BY tt.ticket_count DESC, t.name
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let tags: Vec<TagWithCount> = rows.into_iter().map(|row| {
            let tag = Tag {
                id: row.id,
                name: row.name,
                color: row.color,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };

            TagWithCount {
                tag,
                ticket_count: row.ticket_count,
            }
        }).collect();

        Ok(tags)
    }
}