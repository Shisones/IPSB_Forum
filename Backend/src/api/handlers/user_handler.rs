use crate::models::user_model::*;
use crate::utils::error::AppError;
use crate::services::user_service::UserService;
use axum::{http::StatusCode, Json, extract::State};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn, instrument};
use sqlx::PgPool;

#[instrument(name = "> GET /users", skip_all)]
pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<UserResponse>>, AppError> {
    info!("Fetching all users");
    let users = UserService::get_all(&pool).await?;
    info!("Returning {} users", users.len());
    Ok(Json(users))
}

#[instrument(name = "> POST /users", skip_all)]
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    info!("Creating new user with username: {}", payload.username);
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        warn!("Invalid input: username, email, or password is empty");
        return Err(AppError::BadRequest("All fields are required".to_string()));
    }
    let user = UserService::create(&pool, payload).await?;
    debug!("Created user with ID: {}", user.id);
    Ok((StatusCode::CREATED, Json(user)))
}
