use crate::domain::repositories::Repository;
use crate::web::middleware::ContextExt;
use crate::Context;
use tide::Response;

pub async fn delete_article<R: 'static + Repository + Sync + Send>(
    cx: tide::Request<Context<R>>,
) -> Result<Response, Response> {
    let slug: String = cx.param("slug").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    // They have to be authenticated to perform deletions
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();

    let user = repository.get_user_by_id(user_id)?;
    let article = repository.get_article_by_slug(&slug)?;
    user.delete(article, repository)?;

    Ok(Response::new(200))
}
