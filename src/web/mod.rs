pub mod articles;
pub mod users;
use http::status::StatusCode;
use log::error;
use tide::Error;

pub fn diesel_error(e: &diesel::result::Error) -> Error {
    error!("{}", e);
    Error::from(StatusCode::INTERNAL_SERVER_ERROR)
}
