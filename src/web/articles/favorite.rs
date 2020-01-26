use crate::domain::repositories::Repository;
use crate::web::articles::responses::ArticleResponse;
use crate::web::middleware::ContextExt;
use crate::Context;
use tide::{Request, Response};

pub async fn favorite<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, Response> {
    _favorite(cx, Action::Favorite).await
}

pub async fn unfavorite<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, Response> {
    _favorite(cx, Action::Unfavorite).await
}

pub enum Action {
    Favorite,
    Unfavorite,
}

pub async fn _favorite<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
    action: Action,
) -> Result<Response, Response> {
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let slug: String = cx.param("slug").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    let user = repository.get_user_by_id(user_id)?;
    let article = repository.get_article_by_slug(&slug)?;
    let article_view = match action {
        Action::Favorite => user.favorite(article, repository),
        Action::Unfavorite => user.unfavorite(article, repository),
    }?;

    let response: ArticleResponse = article_view.into();
    Ok(Response::new(200).body_json(&response).unwrap())
}
