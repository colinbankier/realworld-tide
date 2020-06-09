use crate::comments::responses::CommentsResponse;
use crate::ContextExt;
use crate::{Context, ErrorResponse};
use domain::commands::{comments::GetComments, Handle};
use domain::repositories::Repository;
use tide::Response;

pub async fn get<R: 'static + Repository + Sync + Send>(
    cx: tide::Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let article_slug: String = cx.param("slug").map_err(|_| Response::new(400))?;
    let comments = cx.get_handler().handle(GetComments { article_slug });
    let response = CommentsResponse::from(comments?);
    Ok(Response::new(200).body_json(&response).unwrap())
}
