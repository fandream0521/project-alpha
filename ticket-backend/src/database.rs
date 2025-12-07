use sqlx::{PgPool, Pool, Postgres};
use std::env;
use tracing::{error, info};

pub type DbPool = Pool<Postgres>;

// 数据库配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let url =
            env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("DATABASE_URL 环境变量未设置"))?;

        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| anyhow::anyhow!("DATABASE_MAX_CONNECTIONS 必须是有效的数字"))?;

        Ok(Self {
            url,
            max_connections,
        })
    }
}

// 初始化数据库连接池
pub async fn init_database() -> anyhow::Result<DbPool> {
    let config = DatabaseConfig::from_env()?;

    info!("正在连接数据库...");

    let pool = PgPool::connect(&config.url).await?;

    // 设置连接池大小
    // 注意：sqlx 0.7 版本中连接池配置方式可能有所不同

    // 测试数据库连接
    sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
        error!("数据库连接失败: {}", e);
        anyhow::anyhow!("无法连接到数据库: {}", e)
    })?;

    info!("数据库连接成功！");
    Ok(pool)
}

// 运行数据库迁移
pub async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    info!("正在运行数据库迁移...");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            error!("数据库迁移失败: {}", e);
            anyhow::anyhow!("数据库迁移失败: {}", e)
        })?;

    info!("数据库迁移完成！");
    Ok(())
}

// 数据库健康检查
pub async fn health_check(pool: &DbPool) -> bool {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => {
            info!("数据库健康检查通过");
            true
        }
        Err(e) => {
            error!("数据库健康检查失败: {}", e);
            false
        }
    }
}

// 获取数据库统计信息
pub async fn get_database_stats(pool: &DbPool) -> anyhow::Result<DatabaseStats> {
    let tickets_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tickets")
        .fetch_one(pool)
        .await?;

    let tags_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tags")
        .fetch_one(pool)
        .await?;

    let comments_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comments")
        .fetch_one(pool)
        .await?;

    Ok(DatabaseStats {
        tickets_count,
        tags_count,
        comments_count,
    })
}

// 数据库统计信息
#[derive(Debug, serde::Serialize)]
pub struct DatabaseStats {
    pub tickets_count: i64,
    pub tags_count: i64,
    pub comments_count: i64,
}
