use crate::conduit::{articles, comments, users};
use crate::db::models::NewComment;
use crate::middleware::ContextExt;
use crate::web::comments::responses::{Comment, CommentResponse};
use crate::web::diesel_error;
use crate::Repo;
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

pub async fn create(mut cx: tide::Request<Repo>) -> Result<Response, Response> {
    let new_comment: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let author_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let slug: String = cx.param("slug").map_err(|_| Response::new(401))?;
    let repo = cx.state();

    let author = users::find(repo, author_id)?;
    let (article, _, _) = articles::find_one(&repo, &slug).map_err(|e| diesel_error(&e))?;

    let new_comment = NewComment {
        author_id: author.id,
        article_id: article.id,
        body: new_comment.comment.body,
    };

    let comment = comments::create_comment(repo, new_comment).map_err(|e| diesel_error(&e))?;
    let response = CommentResponse {
        comment: Comment::new(comment, author),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
