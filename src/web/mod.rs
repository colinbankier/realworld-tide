pub mod app;
pub mod articles;
pub mod comments;
pub mod profiles;
pub mod users;

use http::status::StatusCode;
use log::error;
use tide::Error;

pub use app::get_app;

pub fn internal_error<E: std::fmt::Display>(e: E) -> Error {
    error!("{}", e);
    Error::from(StatusCode::INTERNAL_SERVER_ERROR)
}
