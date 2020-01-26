pub mod app;
pub mod articles;
pub mod auth;
pub mod comments;
pub mod middleware;
pub mod profiles;
pub mod users;

use tide::Response;

use crate::domain::{
    ChangeArticleError, DatabaseError, DeleteCommentError, GetArticleError, GetUserError,
    LoginError, PublishArticleError, SignUpError,
};
pub use app::get_app;

impl From<GetUserError> for Response {
    fn from(e: GetUserError) -> Response {
        match &e {
            GetUserError::NotFound { .. } => Response::new(404).body_string(e.to_string()),
            GetUserError::DatabaseError(_) => Response::new(500),
        }
    }
}

impl From<LoginError> for Response {
    fn from(e: LoginError) -> Response {
        match &e {
            LoginError::NotFound => Response::new(401),
            LoginError::DatabaseError(_) => Response::new(500),
        }
    }
}

impl From<SignUpError> for Response {
    fn from(e: SignUpError) -> Response {
        match &e {
            SignUpError::DatabaseError(_) => Response::new(500),
        }
    }
}

impl From<GetArticleError> for Response {
    fn from(e: GetArticleError) -> Response {
        match &e {
            GetArticleError::ArticleNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            GetArticleError::DatabaseError(_) => Response::new(500),
        }
    }
}

impl From<DatabaseError> for Response {
    fn from(_: DatabaseError) -> Response {
        Response::new(500)
    }
}

impl From<PublishArticleError> for Response {
    fn from(e: PublishArticleError) -> Response {
        match &e {
            PublishArticleError::AuthorNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            PublishArticleError::DuplicatedSlug { .. } => {
                Response::new(400).body_string(e.to_string())
            }
            PublishArticleError::DatabaseError(_) => Response::new(500),
        }
    }
}

impl From<ChangeArticleError> for Response {
    fn from(e: ChangeArticleError) -> Response {
        match &e {
            ChangeArticleError::ArticleNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            ChangeArticleError::Forbidden { .. } => Response::new(401).body_string(e.to_string()),
            ChangeArticleError::DatabaseError(_) => Response::new(500),
        }
    }
}

impl From<DeleteCommentError> for Response {
    fn from(e: DeleteCommentError) -> Response {
        match &e {
            DeleteCommentError::CommentNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            DeleteCommentError::Forbidden { .. } => Response::new(401).body_string(e.to_string()),
            DeleteCommentError::DatabaseError(_) => Response::new(500),
        }
    }
}
