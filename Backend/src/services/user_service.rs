use crate::models::user_model::{UserResponse, CreateUserRequest};
use crate::utils::error::AppError;
use sqlx::PgPool;
use tracing::{info, debug};

pub struct UserService;

impl UserService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<UserResponse>, AppError> {
        info!("Service: Fetching all users");
        let users = sqlx::query_as!(
            UserResponse,
            "SELECT id, username, email FROM users ORDER BY id"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to fetch users: {}", e)))?;
        debug!("Service: Retrieved {} users", users.len());
        Ok(users)
    }

    pub async fn create(pool: &PgPool, payload: CreateUserRequest) -> Result<UserResponse, AppError> {
        info!("Service: Creating user with username: {}", payload.username);
        let user = sqlx::query_as!(
            UserResponse,
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email",
            payload.username,
            payload.email,
            payload.password
        )
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(ref dbe) if dbe.constraint().is_some() => {
                AppError::BadRequest("Username or email already exists".to_string())
            }
            _ => AppError::Internal(format!("Failed to create user: {}", e)),
        })?;
        debug!("Service: Created user with ID: {}", user.id);
        Ok(user)
    }
}
