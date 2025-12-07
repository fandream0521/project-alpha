use std::net::SocketAddr;
use tracing::info;
use ticket_backend::{config::Config, database::init_database, routes::create_app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();

    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let config = Config::from_env();

    // 初始化数据库连接池
    let pool = init_database().await?;

    // 创建应用路由
    let app = create_app(pool);

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    info!("Starting server on {}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
