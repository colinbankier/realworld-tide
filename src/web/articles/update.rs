use crate::domain::repositories::{ArticleRepository, UsersRepository};
use crate::domain::ArticleUpdate;
use crate::middleware::ContextExt;
use crate::web::articles::responses::ArticleResponse;
use crate::Repo;
use serde::{Deserialize, Serialize};
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

pub async fn update_article(mut cx: tide::Request<Repo>) -> Result<Response, Response> {
    let request: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let slug: String = cx.param("slug").map_err(|_| Response::new(401))?;
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let article = repository.get_by_slug(&slug)?;
    let user = repository.get_by_id(user_id)?;
    let updated_article = user.update(article, request.into(), &repository)?;

    let response: ArticleResponse = repository.get_article_view(&user, updated_article)?.into();
    Ok(Response::new(200).body_json(&response).unwrap())
}
