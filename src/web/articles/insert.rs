use crate::conduit::{articles, users};
use crate::db::models::NewArticle;
use crate::middleware::ContextExt;
use crate::web::articles::responses::{Article, ArticleResponse};
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use itertools::Itertools;
use serde::Deserialize;
use tide::Response;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub article: NewArticleRequest,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleRequest {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

pub async fn insert_article(mut cx: tide::Request<Repo>) -> tide::Result<Response> {
    let new_article: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let auth = cx.get_claims().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let repo = cx.state();

    let user = users::find(repo, auth.user_id()).map_err(|e| diesel_error(&e))?;
    let new_article = NewArticle {
        description: new_article.article.description,
        slug: get_slug(new_article.article.title.clone()),
        title: new_article.article.title,
        body: new_article.article.body,
        tag_list: new_article.article.tag_list,
        user_id: user.id,
    };

    let article = articles::insert(repo, new_article).map_err(|e| diesel_error(&e))?;
    let response = ArticleResponse {
        article: Article::new(article, user, 0),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}

/// Convert a title into a url-safe slug
fn get_slug(title: String) -> String {
    title
        .to_ascii_lowercase()
        .split_ascii_whitespace()
        .join("-")
}
