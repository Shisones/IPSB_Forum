use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::reply_model::Reply;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub votes: i32,
    pub replies: Vec<Reply>,
    pub category: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub votes: i32,
    pub replies: Vec<Reply>,
    pub category: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub desc: String,
    pub author: String,
    pub category: String,
    pub tags: Vec<String>,
}

