use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub checked: bool,
}

#[derive(Deserialize)]
pub struct PostRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct PostUpdateRequest {
    pub name: String,
    pub checked: bool,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub name: String,
    pub checked: bool,
}

impl PostResponse {
    pub fn of(post: Post) -> PostResponse {
        PostResponse {
            id: post.id,
            name: post.name,
            checked: post.checked,
        }
    }
}