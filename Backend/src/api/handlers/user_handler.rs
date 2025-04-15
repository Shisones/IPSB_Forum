use crate::models::user_model::*;
use crate::utils::error::AppError;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn, instrument};

#[instrument(name = "> GET /users", skip_all)]
pub async fn get_users() -> Result<Json<Vec<UserResponse>>, AppError> {
    info!("Fetching all users");

    // Mock data for demonstration (no database)
    let users = vec![
        UserResponse {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
        },
        UserResponse {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
        },
    ];

    info!("Returning {} users", users.len());
    Ok(Json(users))
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    info!("Creating new user with username: {}", payload.username);

    // Validate input
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        warn!("Invalid input: username, email, or password is empty");
        return Err(AppError::BadRequest("All fields are required".to_string()));
    }

    // Mock user creation (no database)
    let new_user = UserResponse {
        id: 3, // Simulated ID
        username: payload.username,
        email: payload.email,
    };

    debug!("Created user with ID: {}", new_user.id);
    Ok((StatusCode::CREATED, Json(new_user)))
}
