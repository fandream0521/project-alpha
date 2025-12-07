// 数据库连接测试程序
use dotenv::dotenv;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载环境变量
    dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("=== 数据库连接测试 ===\n");

    // 测试数据库连接
    println!("1. 测试数据库连接...");
    match ticket_backend::database::init_database().await {
        Ok(pool) => {
            println!("✅ 数据库连接成功！");

            // 测试健康检查
            println!("2. 测试健康检查...");
            if ticket_backend::database::health_check(&pool).await {
                println!("✅ 健康检查通过！");
            } else {
                println!("❌ 健康检查失败！");
                return Ok(());
            }

            // 测试统计信息
            println!("3. 获取数据库统计信息...");
            match ticket_backend::database::get_database_stats(&pool).await {
                Ok(stats) => {
                    println!("✅ 统计信息：");
                    println!("   - 工单数量: {}", stats.tickets_count);
                    println!("   - 标签数量: {}", stats.tags_count);
                    println!("   - 评论数量: {}", stats.comments_count);
                }
                Err(e) => {
                    println!("❌ 获取统计信息失败: {}", e);
                }
            }

            // 测试数据模型
            println!("4. 测试数据模型...");
            test_data_models(&pool).await?;
        }
        Err(e) => {
            println!("❌ 数据库连接失败: {}", e);
            return Err(e);
        }
    }

    println!("\n=== 所有测试完成 ===");
    Ok(())
}

// 测试数据模型
async fn test_data_models(pool: &ticket_backend::database::DbPool) -> anyhow::Result<()> {
    use ticket_backend::models::{Priority, TicketStatus};
    use uuid::Uuid;

    // 测试插入标签
    println!("   - 测试插入标签...");
    let tag_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO tags (id, name, color) VALUES ($1, $2, $3)",
        tag_id,
        "test-tag",
        "#FF0000"
    )
    .execute(pool)
    .await?;

    // 测试查询标签
    println!("   - 测试查询标签...");
    let tag: Option<ticket_backend::models::Tag> =
        sqlx::query_as("SELECT id, name, color, created_at, updated_at FROM tags WHERE id = $1")
            .bind(tag_id)
            .fetch_optional(pool)
            .await?;

    if tag.is_some() {
        println!("     ✅ 标签查询成功！");
    } else {
        println!("     ❌ 标签查询失败！");
    }

    // 测试插入工单
    println!("   - 测试插入工单...");
    let ticket_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO tickets (id, title, description, status, priority) VALUES ($1, $2, $3, $4, $5)",
        ticket_id,
        "Test Ticket",
        "This is a test ticket",
        TicketStatus::Open as TicketStatus,
        Priority::Medium as Priority
    )
    .execute(pool)
    .await?;

    // 测试查询工单
    println!("   - 测试查询工单...");
    let ticket: Option<ticket_backend::models::Ticket> = sqlx::query_as(
        "SELECT id, title, description, status, priority, assignee_id, reporter_id, created_at, updated_at, resolved_at FROM tickets WHERE id = $1"
    )
    .bind(ticket_id)
    .fetch_optional(pool)
    .await?;

    if ticket.is_some() {
        println!("     ✅ 工单查询成功！");
    } else {
        println!("     ❌ 工单查询失败！");
    }

    // 清理测试数据
    println!("   - 清理测试数据...");
    sqlx::query!("DELETE FROM ticket_tags WHERE ticket_id = $1", ticket_id)
        .execute(pool)
        .await?;

    sqlx::query!("DELETE FROM comments WHERE ticket_id = $1", ticket_id)
        .execute(pool)
        .await?;

    sqlx::query!("DELETE FROM tickets WHERE id = $1", ticket_id)
        .execute(pool)
        .await?;

    sqlx::query!("DELETE FROM tags WHERE id = $1", tag_id)
        .execute(pool)
        .await?;

    println!("     ✅ 测试数据清理完成！");
    println!("   ✅ 数据模型测试完成！");

    Ok(())
}
