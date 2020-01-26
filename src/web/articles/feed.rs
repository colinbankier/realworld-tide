use crate::domain::repositories::Repository;
use crate::middleware::ContextExt;
use crate::web::articles::responses::ArticlesResponse;
use crate::Context;
use serde;
use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[derive(Serialize, Deserialize)]
pub struct FeedQuery {
    #[serde(default)]
    pub limit: u64,

    #[serde(default)]
    pub offset: u64,
}

impl Default for FeedQuery {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
        }
    }
}

impl From<FeedQuery> for crate::domain::FeedQuery {
    fn from(f: FeedQuery) -> Self {
        Self {
            limit: f.limit,
            offset: f.offset,
        }
    }
}

pub async fn feed<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, Response> {
    // This can be avoided once https://github.com/http-rs/tide/pull/384 gets merged
    let query = cx.query::<FeedQuery>().unwrap_or_default();
    let repository = &cx.state().repository;

    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let user = repository.get_user_by_id(user_id)?;

    let articles = user.feed(query.into(), repository)?;
    let response = ArticlesResponse::from(articles);
    Ok(Response::new(200).body_json(&response).unwrap())
}
