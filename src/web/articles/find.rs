use crate::domain::repositories::Repository;
use crate::web::articles::responses::ArticleResponse;
use crate::web::middleware::ContextExt;
use crate::Context;
use tide::{Request, Response};
use uuid::Uuid;

pub async fn get_article<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, Response> {
    let slug: String = cx.param("slug").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    let article = repository.get_article_by_slug(&slug)?;
    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let response: ArticleResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id).unwrap();
            let article_view = repository.get_article_view(&user, article).unwrap();
            article_view.into()
        }
        None => article.into(),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
