use crate::middleware::ContextExt;
use crate::web::articles::responses::ArticleResponse;
use crate::{domain, Repo};
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tide::Response;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub article: NewArticleRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleRequest {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

impl Into<domain::ArticleContent> for NewArticleRequest {
    fn into(self) -> domain::ArticleContent {
        domain::ArticleContent::new(self.title, self.description, self.body, self.tag_list)
    }
}

pub async fn insert_article(mut cx: tide::Request<Repo>) -> tide::Result<Response> {
    let new_article: Request = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let author_id = cx
        .get_claims()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .user_id();
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let article_draft = domain::ArticleDraft::new(new_article.article.into(), author_id);
    let published_article = article_draft.publish(&repository);

    Ok(Response::new(200)
        .body_json(&ArticleResponse::from(published_article))
        .unwrap())
}
