use crate::conduit::favorites::{is_favorite, n_favorites};
use crate::conduit::{articles, users};
use crate::db::models::UpdateArticle;
use crate::middleware::ContextExt;
use crate::web::articles::responses::{Article, ArticleResponse};
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tide::{Response, ResultExt};

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

pub async fn update_article(mut cx: tide::Request<Repo>) -> tide::Result<Response> {
    let request: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let slug: String = cx.param("slug").client_err()?;
    let auth = cx.get_claims().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let repo = cx.state();

    let article_update = UpdateArticle {
        title: request.article.title,
        body: request.article.body,
        description: request.article.description,
    };
    let updated_article =
        articles::update(repo, article_update, slug).map_err(|e| diesel_error(&e))?;
    let author = users::find(&repo, updated_article.user_id).map_err(|e| diesel_error(&e))?;
    let n_fav = n_favorites(&repo, updated_article.id).map_err(|e| diesel_error(&e))?;
    let is_fav =
        is_favorite(&repo, auth.user_id(), updated_article.id).map_err(|e| diesel_error(&e))?;

    let response = ArticleResponse {
        article: Article::new(updated_article, author, n_fav, is_fav),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
