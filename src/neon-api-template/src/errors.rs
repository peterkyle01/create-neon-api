use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("password hash error: {0}")]
    Argon2(argon2::password_hash::Error),

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    Authentication(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    Internal(String),
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        AppError::Argon2(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Http(e) => {
                tracing::error!("data api error: {:?}", e);
                (StatusCode::BAD_GATEWAY, "upstream request failed")
            }
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "invalid token"),
            AppError::Argon2(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.as_str()),
            AppError::Internal(msg) => {
                tracing::error!("internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
