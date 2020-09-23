use crate::articles::responses::ArticleResponse;
use crate::middleware::ContextExt;
use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use domain::ArticleUpdate;
use serde::{Deserialize, Serialize};
use tide::prelude::*;
use tide::Response;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub article: UpdateArticleRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

impl From<Request> for ArticleUpdate {
    fn from(r: Request) -> ArticleUpdate {
        ArticleUpdate {
            title: r.article.title,
            body: r.article.body,
            description: r.article.description,
        }
    }
}

pub async fn update_article<R: 'static + Repository + Sync + Send>(
    mut cx: tide::Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let request: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::builder(400).body(e.to_string()))?;
    let slug: String = cx.param("slug").map_err(|_| Response::new(401))?;
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = &cx.state().repository;

    let article = repository.get_article_by_slug(&slug)?;
    let user = repository.get_user_by_id(user_id)?;
    let updated_article = user.update_article(article, request.into(), repository)?;

    let response: ArticleResponse = repository.get_article_view(&user, updated_article)?.into();
    Ok(Response::builder(200).body(json!(&response)).into())
}
