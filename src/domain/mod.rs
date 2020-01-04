use chrono::{DateTime, Utc};
use derive_more::Constructor;
use getset::Getters;
use itertools::Itertools;
use uuid::Uuid;

#[derive(Getters, Clone, Constructor, Debug)]
#[get = "pub"]
pub struct ArticleContent {
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}

#[derive(Getters, Clone, Constructor, Debug)]
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

    pub fn publish(
        self,
        repository: &impl ArticleRepository,
    ) -> Result<Article, PublishArticleError> {
        repository.publish(self)
    }
}

#[derive(Getters, Clone, Constructor, Debug)]
#[get = "pub"]
pub struct Article {
    content: ArticleContent,
    slug: String,
    author: Profile,
    metadata: ArticleMetadata,
    favorites_count: u64,
}

#[derive(Getters, Clone, Constructor, Debug)]
#[get = "pub"]
pub struct ArticleMetadata {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Getters, Clone, Constructor, Debug)]
#[get = "pub"]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

#[derive(Getters, Clone, Constructor, Debug)]
#[get = "pub"]
pub struct User {
    id: Uuid,
    email: String,
    profile: Profile,
}

#[derive(thiserror::Error, Debug)]
pub enum PublishArticleError {
    #[error("There is no author with user id {author_id:?}.")]
    AuthorNotFound {
        author_id: Uuid,
        #[source]
        source: GetUserError,
    },
    #[error("There is already an article using {slug:?} as slug. Change title!")]
    DuplicatedSlug {
        slug: String,
        #[source]
        source: diesel::result::Error,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] diesel::result::Error),
}

impl From<GetUserError> for PublishArticleError {
    fn from(e: GetUserError) -> Self {
        match e {
            GetUserError::NotFound { user_id, source: _ } => PublishArticleError::AuthorNotFound {
                author_id: user_id,
                source: e,
            },
            e => e.into(),
        }
    }
}

pub trait ArticleRepository {
    fn publish(&self, draft: ArticleDraft) -> Result<Article, PublishArticleError>;
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserError {
    #[error("There is no user with id {user_id:?}.")]
    NotFound {
        user_id: Uuid,
        source: diesel::result::Error,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] diesel::result::Error),
}

pub trait UsersRepository {
    fn get_by_id(&self, user_id: Uuid) -> Result<User, GetUserError>;
}
