use crate::models::post_model::*;
use crate::utils::error::AppError;
use axum::{
    extract::{Path, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::{instrument, info, debug, warn};
use chrono::Utc;

// Mock data for demonstration (no database)
fn get_mock_posts() -> Vec<Post> {
    vec![Post {
        id: 1,
        title: "Project Gentian - Infinity".to_string(),
        desc: "Project that has something something lorem ipsum".to_string(),
        author: "JohnDoe".to_string(),
        date: Utc::now(),
        votes: 130,
        replies: vec![],
        category: "Showcase".to_string(),
        tags: vec!["mod".to_string(), "tutorial".to_string()],
    }]
}

#[instrument(name = "> POST /posts", skip_all)]
pub async fn create_post(Json(payload): Json<CreatePostRequest>) -> Result<(StatusCode, Json<PostResponse>), AppError> {
    info!("Creating new post with title: {}", payload.title);

    // Validate input
    if payload.title.is_empty() || payload.desc.is_empty() || payload.author.is_empty() {
        warn!("Invalid input: title, desc, or author is empty");
        return Err(AppError::BadRequest("Title, description, and author are required".to_string()));
    }

    // Mock post creation (no database)
    let new_post = PostResponse {
        id: 2, // Simulated ID
        title: payload.title,
        desc: payload.desc,
        author: payload.author,
        date: Utc::now(),
        votes: 0,
        replies: vec![],
        category: payload.category,
        tags: payload.tags,
    };

    debug!("Created post with ID: {}", new_post.id);
    Ok((StatusCode::CREATED, Json(new_post)))
}

#[instrument(name = "> GET /posts", skip_all)]
pub async fn read_post_all() -> Result<Json<Vec<PostResponse>>, AppError> {
    info!("Fetching all posts");

    // Mock data
    let posts = get_mock_posts();
    let response: Vec<PostResponse> = posts.into_iter().map(|p| PostResponse {
        id: p.id,
        title: p.title,
        desc: p.desc,
        author: p.author,
        date: p.date,
        votes: p.votes,
        replies: p.replies,
        category: p.category,
        tags: p.tags,
    }).collect();

    info!("Returning {} posts", response.len());
    Ok(Json(response))
}

#[instrument(name = "> GET /posts/:id", skip_all)]
pub async fn read_post(Path(id): Path<i32>) -> Result<Json<PostResponse>, AppError> {
    info!("Fetching post with ID: {}", id);

    // Mock data
    let posts = get_mock_posts();
    let post = posts.into_iter().find(|p| p.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Post with ID {} not found", id)))?;

    let response = PostResponse {
        id: post.id,
        title: post.title,
        desc: post.desc,
        author: post.author,
        date: post.date,
        votes: post.votes,
        replies: post.replies,
        category: post.category,
        tags: post.tags,
    };

    info!("Found post with ID: {}", id);
    Ok(Json(response))
}

#[instrument(name = "> PUT /posts/:id", skip_all)]
pub async fn update_post(
    Path(id): Path<i32>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<PostResponse>), AppError> {
    info!("Updating post with ID: {}", id);

    // Validate input
    if payload.title.is_empty() || payload.desc.is_empty() || payload.author.is_empty() {
        warn!("Invalid input: title, desc, or author is empty");
        return Err(AppError::BadRequest("Title, description, and author are required".to_string()));
    }

    // Mock data
    let posts = get_mock_posts();
    let post = posts.into_iter().find(|p| p.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Post with ID {} not found", id)))?;

    // Simulate update
    let updated_post = PostResponse {
        id,
        title: payload.title,
        desc: payload.desc,
        author: payload.author,
        date: Utc::now(),
        votes: post.votes,
        replies: post.replies,
        category: payload.category,
        tags: payload.tags,
    };

    debug!("Updated post with ID: {}", id);
    Ok((StatusCode::OK, Json(updated_post)))
}

#[instrument(name = "> DELETE /posts/:id", skip_all)]
pub async fn delete_post(Path(id): Path<i32>) -> Result<StatusCode, AppError> {
    info!("Deleting post with ID: {}", id);

    // Mock data
    let posts = get_mock_posts();
    if !posts.iter().any(|p| p.id == id) {
        warn!("Post with ID {} not found", id);
        return Err(AppError::NotFound(format!("Post with ID {} not found", id)));
    }

    debug!("Deleted post with ID: {}", id);
    Ok(StatusCode::NO_CONTENT)
}
