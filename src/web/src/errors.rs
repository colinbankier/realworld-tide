//! A sub-module to prescribe how each domain error gets converted to an HTTP response.
use crate::ErrorResponse;
use domain::{
    ChangeArticleError, DatabaseError, DeleteCommentError, GetArticleError, GetUserError,
    LoginError, PasswordError, PublishArticleError, SignUpError,
};
use tide::Response;

impl From<GetUserError> for ErrorResponse {
    fn from(e: GetUserError) -> ErrorResponse {
        let r = match &e {
            GetUserError::NotFound { .. } => Response::new(404).body_string(e.to_string()),
            GetUserError::DatabaseError(_) => Response::new(500),
        };
        ErrorResponse(r)
    }
}

impl From<PasswordError> for ErrorResponse {
    fn from(_: PasswordError) -> ErrorResponse {
        ErrorResponse(Response::new(500))
    }
}

impl From<LoginError> for ErrorResponse {
    fn from(e: LoginError) -> ErrorResponse {
        let r = match &e {
            LoginError::NotFound => Response::new(401),
            LoginError::PasswordError(_) => Response::new(500),
            LoginError::DatabaseError(_) => Response::new(500),
        };
        ErrorResponse(r)
    }
}

impl From<SignUpError> for ErrorResponse {
    fn from(e: SignUpError) -> ErrorResponse {
        let r = match &e {
            SignUpError::DatabaseError(_) => Response::new(500),
        };
        ErrorResponse(r)
    }
}

impl From<GetArticleError> for ErrorResponse {
    fn from(e: GetArticleError) -> ErrorResponse {
        let r = match &e {
            GetArticleError::ArticleNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            GetArticleError::DatabaseError(_) => Response::new(500),
        };
        ErrorResponse(r)
    }
}

impl From<DatabaseError> for ErrorResponse {
    fn from(_: DatabaseError) -> ErrorResponse {
        ErrorResponse(Response::new(500))
    }
}

impl From<PublishArticleError> for ErrorResponse {
    fn from(e: PublishArticleError) -> ErrorResponse {
        let r = match &e {
            PublishArticleError::AuthorNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            PublishArticleError::DuplicatedSlug { .. } => {
                Response::new(400).body_string(e.to_string())
            }
            PublishArticleError::DatabaseError(_) => Response::new(500),
        };
        ErrorResponse(r)
    }
}

impl From<ChangeArticleError> for ErrorResponse {
    fn from(e: ChangeArticleError) -> ErrorResponse {
        let r = match &e {
            ChangeArticleError::ArticleNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            ChangeArticleError::Forbidden { .. } => Response::new(401).body_string(e.to_string()),
            ChangeArticleError::DatabaseError(_) => Response::new(500),
        };
        ErrorResponse(r)
    }
}

impl From<DeleteCommentError> for ErrorResponse {
    fn from(e: DeleteCommentError) -> ErrorResponse {
        let r = match &e {
            DeleteCommentError::CommentNotFound { .. } => {
                Response::new(404).body_string(e.to_string())
            }
            DeleteCommentError::Forbidden { .. } => Response::new(401).body_string(e.to_string()),
            DeleteCommentError::DatabaseError(_) => Response::new(500),
        };
        ErrorResponse(r)
    }
}
