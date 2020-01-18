use crate::domain::articles::ArticleQuery;
use crate::domain::repositories::{ArticleRepository, UsersRepository};
use crate::middleware::ContextExt;
use crate::web::articles::responses::ArticlesResponse;
use crate::Repo;
use tide::{Request, Response};
use uuid::Uuid;

pub async fn list_articles(cx: Request<Repo>) -> Result<Response, Response> {
    // This can be avoided once https://github.com/http-rs/tide/pull/384 gets merged
    let query = cx.query::<ArticleQuery>().unwrap_or(ArticleQuery {
        author: None,
        favorited: None,
        tag: None,
    });

    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let articles = repository.find_articles(query)?;
    let response: ArticlesResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_by_id(user_id)?;
            let views = repository.get_articles_views(&user, articles)?;
            ArticlesResponse::from(views)
        }
        None => ArticlesResponse::from(articles),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
