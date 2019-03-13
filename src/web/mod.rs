pub mod articles;
pub mod users;
use http::status::StatusCode;
use log::error;

pub fn diesel_error(e: &diesel::result::Error) -> StatusCode {
    error!("{}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}
