use crate::articles::responses::Author;
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

impl From<domain::Comment> for Comment {
    fn from(c: domain::Comment) -> Self {
        Self {
            id: c.id,
            body: c.body,
            created_at: c.created_at,
            updated_at: c.updated_at,
            author: c.author.into(),
        }
    }
}

impl From<domain::CommentView> for Comment {
    fn from(c: domain::CommentView) -> Self {
        Self {
            id: c.id,
            body: c.body,
            created_at: c.created_at,
            updated_at: c.updated_at,
            author: c.author.into(),
        }
    }
}

impl<T: Into<Comment>> From<Vec<T>> for CommentsResponse {
    fn from(v: Vec<T>) -> Self {
        let comments: Vec<Comment> = v.into_iter().map(|c| c.into()).collect();
        Self { comments }
    }
}
