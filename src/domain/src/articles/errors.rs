use crate::{DatabaseError, GetUserError};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum GetArticleError {
    #[error("There is no article with {slug:?} as slug.")]
    ArticleNotFound {
        slug: String,
        #[source]
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeArticleError {
    #[error("There is no article with {slug:?} as slug.")]
    ArticleNotFound {
        slug: String,
        #[source]
        source: DatabaseError,
    },
    #[error("User {user_id:?} is not the author of the article (slug: {slug:?}).")]
    Forbidden { user_id: Uuid, slug: String },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
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
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
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
