use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    Internal(String),
    UnprocessableEntity(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };

        let body = ErrorResponse {
            error: message.to_string(),
        };

        (status, Json(body)).into_response()
    }
}
