use crate::conduit::{articles, articles::ArticleQuery};
use crate::models::*;
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use serde::Serialize;
use tide::{Error, Request, Response, ResultExt};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    articles: Vec<Article>,
    articles_count: u64,
}

pub async fn list_articles(cx: Request<Repo>) -> tide::Result<Response> {
    let query = cx.query::<ArticleQuery>()?;
    let repo = cx.state();
    let result = articles::find(repo, query);

    match result {
        Ok(articles) => {
            let articles_count = articles.len() as u64;
            let response = ArticleResponse {
                articles,
                articles_count,
            };
            Ok(Response::new(200).body_json(&response).unwrap())
        }
        Err(e) => Err(diesel_error(&e)),
    }
}

pub async fn get_article(cx: Request<Repo>) -> tide::Result<Response> {
    let slug: String = cx.param("slug").client_err()?;
    let repo = cx.state();
    let result = articles::find_one(repo, &slug);
    match result {
        Ok(b) => Ok(Response::new(200).body_json(&b).unwrap()),
        Err(diesel::NotFound) => Err(Error::from(StatusCode::NOT_FOUND)),
        Err(e) => Err(diesel_error(&e)),
    }
}
