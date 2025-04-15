use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    pub id: i32,
    pub author: String,
    pub contents: String,
    pub date: DateTime<Utc>,
    pub votes: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateReplyRequest {
    pub author: String,
    pub contents: String,
}
