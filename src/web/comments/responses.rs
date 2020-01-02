use crate::db::models;
use crate::web::articles::responses::Author;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponse {
    pub comment: Comment,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub body: String,
    pub author: Author,
}

impl Comment {
    pub fn new(c: models::Comment, a: models::User) -> Self {
        Self {
            id: c.id as u64,
            body: c.body,
            created_at: c.created_at,
            updated_at: c.updated_at,
            author: Author {
                username: a.username,
                bio: a.bio,
                image: a.image,
            },
        }
    }
}
