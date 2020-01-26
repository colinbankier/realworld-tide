use crate::repositories::Repository;
use crate::{Comment, DatabaseError, Profile, ProfileView};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleContent {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

impl ArticleContent {
    /// Convert a title into a url-safe slug
    pub fn slug(&self) -> String {
        self.title
            .to_ascii_lowercase()
            .split_ascii_whitespace()
            .join("-")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Article {
    pub content: ArticleContent,
    pub slug: String,
    pub author: Profile,
    pub metadata: ArticleMetadata,
    pub favorites_count: u64,
}

impl Article {
    pub fn comments(&self, repository: &impl Repository) -> Result<Vec<Comment>, DatabaseError> {
        repository.get_comments(&self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleView {
    pub content: ArticleContent,
    pub slug: String,
    pub author: ProfileView,
    pub metadata: ArticleMetadata,
    pub favorited: bool,
    pub favorites_count: u64,
    // The user owning this view of an article
    pub viewer: Uuid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArticleQuery {
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub tag: Option<String>,
}
