use crate::conduit::articles;
use crate::conduit::favorites;
use crate::middleware::ContextExt;
use crate::web::articles::responses::{Article, ArticleResponse};
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use tide::{Request, Response, ResultExt};

pub async fn favorite(cx: Request<Repo>) -> tide::Result<Response> {
    let user_id = cx
        .get_claims()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .user_id();
    let slug: String = cx.param("slug").client_err()?;
    let repo = cx.state();

    let (article, author, _) = articles::find_one(repo, &slug).map_err(|e| diesel_error(&e))?;
    favorites::favorite(&repo, user_id, article.id).map_err(|e| diesel_error(&e))?;
    let n_favorites = favorites::n_favorites(&repo, article.id).map_err(|e| diesel_error(&e))?;

    let response = ArticleResponse {
        article: Article::new(article, author, n_favorites),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
