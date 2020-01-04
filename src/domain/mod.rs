use chrono::{DateTime, Utc};
use derive_more::Constructor;
use getset::Getters;
use itertools::Itertools;
use uuid::Uuid;

#[derive(Getters, Clone, Constructor)]
#[get = "pub"]
pub struct ArticleContent {
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}

#[derive(Getters, Clone, Constructor)]
#[get = "pub"]
pub struct ArticleDraft {
    content: ArticleContent,
    author_id: Uuid,
}

impl ArticleDraft {
    /// Convert a title into a url-safe slug
    pub fn slug(&self) -> String {
        self.content
            .title
            .to_ascii_lowercase()
            .split_ascii_whitespace()
            .join("-")
    }

    pub fn publish(self, repository: &impl ArticleRepository) -> Article {
        repository.publish(self)
    }
}

#[derive(Getters, Clone, Constructor)]
#[get = "pub"]
pub struct Article {
    content: ArticleContent,
    slug: String,
    author: Profile,
    metadata: ArticleMetadata,
    favorites_count: u64,
}

#[derive(Getters, Clone, Constructor)]
#[get = "pub"]
pub struct ArticleMetadata {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Getters, Clone, Constructor)]
#[get = "pub"]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

#[derive(Getters, Clone, Constructor)]
#[get = "pub"]
pub struct User {
    id: Uuid,
    email: String,
    profile: Profile,
}

pub trait ArticleRepository {
    fn publish(&self, draft: ArticleDraft) -> Article;
}

pub trait UsersRepository {
    fn get_by_id(&self, user_id: Uuid) -> User;
}
