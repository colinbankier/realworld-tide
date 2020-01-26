use crate::domain::repositories::{ArticleRepository, UsersRepository};
use crate::middleware::ContextExt;
use crate::Repo;
use tide::Response;

pub async fn delete(cx: tide::Request<Repo>) -> Result<Response, Response> {
    let author_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let comment_id: u64 = cx.param("id").map_err(|_| Response::new(400))?;
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let author = repository.get_by_id(author_id)?;
    let comment = repository.get_comment(comment_id)?;
    author.delete_comment(comment, &repository)?;

    Ok(Response::new(200))
}
