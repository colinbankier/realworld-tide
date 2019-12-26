use crate::conduit::{articles, articles::ArticleQuery};
use crate::web::articles::responses::ArticleResponse;
use crate::web::diesel_error;
use crate::Repo;
use tide::{Request, Response};

pub async fn list_articles(cx: Request<Repo>) -> tide::Result<Response> {
    let query = cx.query::<ArticleQuery>()?;
    let repo = cx.state();
    let result = articles::find(repo, query);

    match result {
        Ok(articles) => {
            let response = ArticleResponse::new(articles);
            Ok(Response::new(200).body_json(&response).unwrap())
        }
        Err(e) => Err(diesel_error(&e)),
    }
}
