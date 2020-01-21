use crate::domain::ProfileView;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct CommentContent(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Comment {
    pub id: u64,
    pub author: ProfileView,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
