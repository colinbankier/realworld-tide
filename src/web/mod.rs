pub mod articles;
pub mod users;
use http::status::StatusCode;

pub fn diesel_error(e: &diesel::result::Error) -> StatusCode {
    error!("{}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}
