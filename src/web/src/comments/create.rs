use crate::comments::responses::CommentResponse;
use crate::ContextExt;
use crate::{Context, ErrorResponse};
use domain::commands::{comments::CreateComment, Handle};
use domain::repositories::Repository;
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
) -> Result<Response, ErrorResponse> {
    let new_comment: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let article_slug: String = cx.param("slug").map_err(|_| Response::new(400))?;

    let posted_comment = cx.get_handler().handle(CreateComment {
        article_slug,
        comment_body: new_comment.comment.body,
    })?;

    let response = CommentResponse {
        comment: posted_comment.into(),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
