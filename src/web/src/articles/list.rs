use crate::articles::responses::ArticlesResponse;
use crate::middleware::JwtContext;
use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use serde::Deserialize;
use std::str::FromStr;
use tide::{IntoResponse, Request, Response};
use uuid::Uuid;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct ArticleQuery {
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub tag: Option<String>,
}

impl From<ArticleQuery> for domain::ArticleQuery {
    fn from(q: ArticleQuery) -> Self {
        Self {
            author: q.author,
            favorited: q.favorited,
            tag: q.tag,
        }
    }
}

impl FromStr for ArticleQuery {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_urlencoded::from_str::<ArticleQuery>(s).map_err(|e| e.to_string())
    }
}

pub async fn list_articles<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let query = cx.query::<ArticleQuery>().map_err(|e| e.into_response())?;
    let repository = &cx.state().repository;

    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let articles = repository.find_articles(query.into())?;
    let response: ArticlesResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id)?;
            let views = repository.get_articles_views(&user, articles)?;
            ArticlesResponse::from(views)
        }
        None => ArticlesResponse::from(articles),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
