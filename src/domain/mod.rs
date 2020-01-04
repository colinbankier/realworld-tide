use chrono::{DateTime, Utc};
use derive_more::Constructor;
use itertools::Itertools;
use uuid::Uuid;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct ArticleContent {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct ArticleDraft {
    pub content: ArticleContent,
    pub author_id: Uuid,
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

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Article {
    pub content: ArticleContent,
    pub slug: String,
    pub author: Profile,
    pub metadata: ArticleMetadata,
    pub favorites_count: u64,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
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

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct ArticleMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct ProfileView {
    pub profile: Profile,
    pub following: bool,
    // The user owning this view of a profile
    pub viewer: Uuid,
}

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub profile: Profile,
}

impl User {
    pub fn favorite(
        &self,
        article: Article,
        repository: &(impl ArticleRepository + UsersRepository),
    ) -> Result<ArticleView, DatabaseError> {
        let n_favorites = match repository.favorite(&article, self)? {
            FavoriteOutcome::NewFavorite => article.favorites_count + 1,
            FavoriteOutcome::AlreadyAFavorite => article.favorites_count,
        };
        let article_view = ArticleView {
            content: article.content,
            slug: article.slug,
            author: repository.get_view(self, &article.author.username)?,
            metadata: article.metadata,
            favorited: true,
            favorites_count: n_favorites,
            viewer: self.id.to_owned(),
        };
        Ok(article_view)
    }

    pub fn unfavorite(
        &self,
        article: Article,
        repository: &(impl ArticleRepository + UsersRepository),
    ) -> Result<ArticleView, DatabaseError> {
        let n_favorites = match repository.unfavorite(&article, self)? {
            UnfavoriteOutcome::WasAFavorite => article.favorites_count - 1,
            UnfavoriteOutcome::WasNotAFavorite => article.favorites_count,
        };
        let article_view = ArticleView {
            content: article.content,
            slug: article.slug,
            author: repository.get_view(self, &article.author.username)?,
            metadata: article.metadata,
            favorited: false,
            favorites_count: n_favorites,
            viewer: self.id.to_owned(),
        };
        Ok(article_view)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetArticleError {
    #[error("There is no article with {slug:?} as slug.")]
    ArticleNotFound {
        slug: String,
        #[source]
        source: diesel::result::Error,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] diesel::result::Error),
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

pub enum FavoriteOutcome {
    NewFavorite,
    AlreadyAFavorite,
}

pub enum UnfavoriteOutcome {
    WasAFavorite,
    WasNotAFavorite,
}

#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DatabaseError {
    #[from]
    source: diesel::result::Error,
}

impl From<GetUserError> for DatabaseError {
    fn from(e: GetUserError) -> Self {
        match e {
            GetUserError::NotFound { source, .. } => DatabaseError { source },
            GetUserError::DatabaseError(e) => DatabaseError { source: e },
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FavoriteError {
    #[error("The article was already a favorite for the user.")]
    AlreadyAFavorite,
    #[error("Something went wrong.")]
    DatabaseError(#[from] diesel::result::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum UnfavoriteError {
    #[error("The article was not a favorite for the user.")]
    NotAFavoriteBefore,
    #[error("Something went wrong.")]
    DatabaseError(#[from] diesel::result::Error),
}

impl From<GetUserError> for PublishArticleError {
    fn from(e: GetUserError) -> Self {
        match e {
            GetUserError::NotFound { user_id, .. } => PublishArticleError::AuthorNotFound {
                author_id: user_id,
                source: e,
            },
            e => e.into(),
        }
    }
}

pub trait ArticleRepository {
    fn publish(&self, draft: ArticleDraft) -> Result<Article, PublishArticleError>;
    fn get_by_slug(&self, slug: &str) -> Result<Article, GetArticleError>;
    fn get_article_view(
        &self,
        viewer: &User,
        article: Article,
    ) -> Result<ArticleView, GetArticleError>;
    fn favorite(&self, article: &Article, user: &User) -> Result<FavoriteOutcome, DatabaseError>;
    fn unfavorite(
        &self,
        article: &Article,
        user: &User,
    ) -> Result<UnfavoriteOutcome, DatabaseError>;
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
    fn get_view(&self, viewer: &User, username: &str) -> Result<ProfileView, GetUserError>;
}
