use crate::domain::repositories::Repository;
use crate::domain::CommentContent;
use crate::web::comments::responses::CommentResponse;
use crate::web::middleware::ContextExt;
use crate::Context;
use serde::{Deserialize, Serialize};
use tide::Response;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub comment: NewCommentRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewCommentRequest {
    pub body: String,
}

pub async fn create<R: 'static + Repository + Sync + Send>(
    mut cx: tide::Request<Context<R>>,
) -> Result<Response, Response> {
    let new_comment: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let author_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let slug: String = cx.param("slug").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    let author = repository.get_user_by_id(author_id)?;
    let article = repository.get_article_by_slug(&slug)?;
    let posted_comment = author.comment(
        &article,
        CommentContent(new_comment.comment.body),
        repository,
    )?;

    let response = CommentResponse {
        comment: posted_comment.into(),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
