use crate::middleware::JwtContext;
use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use tide::Response;

pub async fn delete<R: 'static + Repository + Sync + Send>(
    cx: tide::Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let author_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let comment_id: u64 = cx.param("id").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    let author = repository.get_user_by_id(author_id)?;
    let comment = repository.get_comment(comment_id)?;
    author.delete_comment(comment, repository)?;

    Ok(Response::new(200))
}
