pub mod app;
pub mod articles;
pub mod comments;
pub mod profiles;
pub mod users;

use log::error;
use tide::Response;

use crate::domain::GetUserError;
pub use app::get_app;

pub fn diesel_error(e: &diesel::result::Error) -> Response {
    error!("{}", e);
    Response::new(500)
}

impl From<GetUserError> for Response {
    fn from(e: GetUserError) -> Response {
        match &e {
            GetUserError::NotFound {
                user_id: _,
                source: _,
            } => Response::new(404).body_string(e.to_string()),
            _ => Response::new(500),
        }
    }
}
