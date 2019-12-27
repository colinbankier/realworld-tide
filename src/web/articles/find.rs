use crate::conduit::articles;
use crate::web::articles::responses::{Article, ArticleResponse};
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use tide::{Error, Request, Response, ResultExt};

pub async fn get_article(cx: Request<Repo>) -> tide::Result<Response> {
    let slug: String = cx.param("slug").client_err()?;
    let repo = cx.state();
    let (article, user, n_favorites) = articles::find_one(repo, &slug).map_err(|e| match e {
        diesel::NotFound => Error::from(StatusCode::NOT_FOUND),
        e => diesel_error(&e),
    })?;
    let response = ArticleResponse {
        article: Article::new(article, user, n_favorites),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
