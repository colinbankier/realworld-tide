use crate::ContextExt;
use crate::{Context, ErrorResponse};
use domain::commands::{comments::DeleteComment, Handle};
use domain::repositories::Repository;
use tide::Response;

pub async fn delete<R: 'static + Repository + Sync + Send>(
    cx: tide::Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let comment_id: u64 = cx.param("id").map_err(|_| Response::new(400))?;
    cx.get_handler().handle(DeleteComment { comment_id })?;

    Ok(Response::new(200))
}
