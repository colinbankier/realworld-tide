use crate::comments::responses::CommentsResponse;
use crate::middleware::ContextExt;
use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use tide::prelude::*;
use tide::Response;
use uuid::Uuid;

pub async fn get<R: 'static + Repository + Sync + Send>(
    cx: tide::Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let slug: String = cx.param("slug").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    let article = repository.get_article_by_slug(&slug)?;
    let comments = article.comments(repository)?;

    let response: CommentsResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id)?;
            let result: Result<Vec<_>, _> = comments
                .into_iter()
                .map(|c| c.view(&user, repository))
                .collect();
            let comment_views = result?;
            CommentsResponse::from(comment_views)
        }
        None => CommentsResponse::from(comments),
    };

    Ok(Response::builder(200).body(json!(&response)).into())
}
