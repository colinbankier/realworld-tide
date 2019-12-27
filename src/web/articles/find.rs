use crate::conduit::articles;
use crate::web::articles::responses::ArticlesResponse;
use crate::web::diesel_error;
use crate::Repo;
use http::status::StatusCode;
use tide::{Error, Request, Response, ResultExt};

pub async fn get_article(cx: Request<Repo>) -> tide::Result<Response> {
    let slug: String = cx.param("slug").client_err()?;
    let repo = cx.state();
    let result = articles::find_one(repo, &slug);
    match result {
        Ok((article, user, n_favorites)) => {
            let response = ArticlesResponse::new(vec![(article, user, n_favorites)]);
            Ok(Response::new(200).body_json(&response).unwrap())
        }
        Err(diesel::NotFound) => Err(Error::from(StatusCode::NOT_FOUND)),
        Err(e) => Err(diesel_error(&e)),
    }
}
