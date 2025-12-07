use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::handlers;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        // 健康检查路由
        .route("/", get(handlers::health_check_basic))
        .route("/health", get(handlers::health_check_basic))
        .route("/api/health", get(handlers::health_check_detailed))
        .route("/api/db/stats", get(handlers::database_stats))
        .route("/api/db/optimize", post(handlers::database_optimize))
        // 标签路由
        .route("/api/v1/tags", get(handlers::list_tags))
        .route("/api/v1/tags", post(handlers::create_tag))
        .route("/api/v1/tags/:id", get(handlers::get_tag))
        .route("/api/v1/tags/:id", put(handlers::update_tag))
        .route("/api/v1/tags/:id", delete(handlers::delete_tag))
        // 工单路由
        .route("/api/v1/tickets", get(handlers::list_tickets))
        .route("/api/v1/tickets", post(handlers::create_ticket))
        .route("/api/v1/tickets/:id", get(handlers::get_ticket))
        .route("/api/v1/tickets/:id", put(handlers::update_ticket))
        .route("/api/v1/tickets/:id", delete(handlers::delete_ticket))
        .layer(CorsLayer::permissive())
        .layer(axum::Extension(pool))
}