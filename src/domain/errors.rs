use crate::domain::{GetArticleError, GetUserError};

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

impl From<GetArticleError> for DatabaseError {
    fn from(e: GetArticleError) -> Self {
        match e {
            GetArticleError::ArticleNotFound { source, .. } => DatabaseError { source },
            GetArticleError::DatabaseError(e) => DatabaseError { source: e },
        }
    }
}
