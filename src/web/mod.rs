pub mod app;
pub mod articles;
pub mod comments;
pub mod profiles;
pub mod users;

use log::error;
use tide::Response;

use crate::domain::{
    ChangeArticleError, DatabaseError, GetArticleError, GetUserError, PublishArticleError,
};
pub use app::get_app;

pub fn diesel_error(e: &diesel::result::Error) -> Response {
    error!("{}", e);
    Response::new(500)
}

impl From<GetUserError> for Response {
    fn from(e: GetUserError) -> Response {
        match &e {
            GetUserError::NotFound { .. } => Response::new(404).body_string(e.to_string()),
            GetUserError::DatabaseError(_) => Response::new(500),
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
