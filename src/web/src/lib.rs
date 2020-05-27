pub mod app;
pub mod articles;
pub mod auth;
pub mod comments;
pub mod errors;
pub mod middleware;
pub mod profiles;
pub mod users;

use crate::auth::Claims;
use domain::commands::CommandHandler;
use domain::repositories::Repository;
use http::status::StatusCode;
use tide::{Error, IntoResponse, Request, Response};
use uuid::Uuid;

pub use app::get_app;

pub trait ContextExt<R: Repository> {
    fn get_handler(&self) -> CommandHandler<R>;
    fn get_claims(&self) -> Result<&Claims, Error>;
}

impl<R> ContextExt<R> for Request<Context<R>>
where
    R: 'static + Repository + Send + Sync,
{
    fn get_handler(&self) -> CommandHandler<R> {
        let author_id: Option<Uuid> = self.get_claims().map(|c| c.user_id()).ok();
        CommandHandler {
            authenticated_user: author_id,
            repository: &self.state().repository,
        }
    }

    fn get_claims(&self) -> Result<&Claims, Error> {
        self.local::<Claims>()
            .ok_or_else(|| Error::from(StatusCode::UNAUTHORIZED))
    }
}

/// The shared state of our application.
/// It's generic with respect to the actual implementation of the repository:
/// this enables swapping different implementations, both for production usage
/// or ease of testing (mocks and stubs).
pub struct Context<R: 'static + Repository + Sync + Send> {
    pub repository: R,
}

/// A wrapper around Tide's Response type.
/// We are leveraging the new-type pattern in order to be able to implement
/// the `From` trait for `ErrorResponse` and the errors defined in the `domain` sub-crate.
pub struct ErrorResponse(pub Response);

/// Convenience conversion.
impl From<Response> for ErrorResponse {
    fn from(e: Response) -> Self {
        Self(e)
    }
}

/// Required to have a Tide-compatible signature for the handler function of each endpoint.
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        self.0
    }
}
