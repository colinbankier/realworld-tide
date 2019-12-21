use crate::conduit::comments;
use crate::middleware::ContextExt;
use crate::web::internal_error;
use crate::Repo;
use http::status::StatusCode;
use tide::{Response, ResultExt};

pub async fn delete(cx: tide::Request<Repo>) -> tide::Result<Response> {
    let author_id = cx
        .get_claims()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .user_id();
    let comment_id: i64 = cx.param("id").client_err()?;
    let repo = cx.state();

    let comment = comments::get_comment(&repo, comment_id).map_err(|e| internal_error(&e))?;
    if author_id != comment.author_id {
        // You can't delete comments written by somebody else
        return Err(tide::Error::from(StatusCode::UNAUTHORIZED));
    }
    comments::delete_comment(repo, comment_id).map_err(|e| internal_error(&e))?;

    Ok(Response::new(200))
}
