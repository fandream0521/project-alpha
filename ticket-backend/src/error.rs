use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("验证错误: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("未找到资源: {0}")]
    NotFound(String),

    #[error("已存在: {0}")]
    Conflict(String),

    #[error("无效请求: {0}")]
    BadRequest(String),

    #[error("内部服务器错误: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("数据库错误: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "数据库操作失败".to_string(),
                )
            }
            AppError::Validation(err) => {
                tracing::warn!("验证错误: {:?}", err);
                (StatusCode::BAD_REQUEST, "输入数据验证失败".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::Internal(msg) => {
                tracing::error!("内部错误: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "内部服务器错误".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "status": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

// 为特定错误类型创建便利构造函数
impl AppError {
    pub fn not_found(resource: &str) -> Self {
        Self::NotFound(format!("{}未找到", resource))
    }

    pub fn conflict(resource: &str) -> Self {
        Self::Conflict(format!("{}已存在", resource))
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}
