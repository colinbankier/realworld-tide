use crate::conduit::{articles, users};
use crate::db::models::NewArticle;
use crate::middleware::ContextExt;
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use serde::Deserialize;
use tide::Response;

#[derive(Deserialize)]
pub struct Request {
    pub article: NewArticleRequest,
}

#[derive(Deserialize)]
pub struct NewArticleRequest {
    pub title: String,
    pub description: String,
    pub body: String,
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
        title: new_article.article.title,
        body: new_article.article.body,
        slug: "".to_string(),
        user_id: user.id,
    };
    let result = articles::insert(repo, new_article);
    match result {
        Ok(b) => Ok(Response::new(200).body_json(&b).unwrap()),
        Err(e) => Err(diesel_error(&e)),
    }
}
