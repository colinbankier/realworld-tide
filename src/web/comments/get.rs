use crate::domain::repositories::Repository;
use crate::middleware::ContextExt;
use crate::web::comments::responses::CommentsResponse;
use crate::Repo;
use tide::Response;
use uuid::Uuid;

pub async fn get(cx: tide::Request<Repo>) -> Result<Response, Response> {
    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let slug: String = cx.param("slug").map_err(|_| Response::new(400))?;
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let article = repository.get_article_by_slug(&slug)?;
    let comments = article.comments(&repository)?;

    let response: CommentsResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id)?;
            let result: Result<Vec<_>, _> = comments
                .into_iter()
                .map(|c| c.view(&user, &repository))
                .collect();
            let comment_views = result?;
            CommentsResponse::from(comment_views)
        }
        None => CommentsResponse::from(comments),
    };

    Ok(Response::new(200).body_json(&response).unwrap())
}
