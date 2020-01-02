use crate::conduit::articles;
use crate::middleware::ContextExt;
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use tide::{Response, ResultExt};

pub async fn delete_article(cx: tide::Request<Repo>) -> tide::Result<Response> {
    let slug: String = cx.param("slug").client_err()?;

    // They have to be authenticated to perform deletions
    cx.get_claims().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let repo = cx.state();

    articles::delete(repo, slug).map_err(|e| diesel_error(&e))?;
    Ok(Response::new(200))
}
