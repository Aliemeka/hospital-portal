use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Invalid Input, cannot be processed: {field} - {message}")]
    UnProcessableEntity { field: String, message: String },
    #[error("Environement Variable is missing: {0}")]
    MissingEnvironmentVarible(String),
    #[error("Failed to Parse: {0}")]
    ParsingError(String),
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::UnProcessableEntity { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("{field} - {message}"),
            ),
            AppError::DatabaseError(msg)
            | AppError::InternalServerError(msg)
            | AppError::MissingEnvironmentVarible(msg)
            | AppError::ParsingError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
