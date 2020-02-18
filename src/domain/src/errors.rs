use crate::{GetArticleError, GetUserError};

#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DatabaseError {
    #[from]
    source: anyhow::Error,
}

impl From<GetUserError> for DatabaseError {
    fn from(e: GetUserError) -> Self {
        match e {
            GetUserError::NotFound { source, .. } => source,
            GetUserError::DatabaseError(e) => e,
        }
    }
}

impl From<GetArticleError> for DatabaseError {
    fn from(e: GetArticleError) -> Self {
        match e {
            GetArticleError::ArticleNotFound { source, .. } => source,
            GetArticleError::DatabaseError(e) => e,
        }
    }
}
