use crate::comments::responses::CommentResponse;
use crate::middleware::ContextExt;
use crate::{Context, ErrorResponse};
use domain::commands::{Command, CommandContext, CreateComment};
use domain::repositories::Repository;
use serde::{Deserialize, Serialize};
use tide::Response;
use uuid::Uuid;

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

    // These block could be implemented as a function on the Tide context, to stay DRY
    let author_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let repository = &cx.state().repository;
    let command_context = CommandContext {
        authenticated_user: author_id,
        repository,
    };

    let posted_comment = CreateComment
    {
        article_slug,
        comment_body: new_comment.comment.body,
    }
    .execute(command_context)
    .expect("Yeah!");

    let response = CommentResponse {
        comment: posted_comment.into(),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
