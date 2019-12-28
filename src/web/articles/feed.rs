use crate::conduit::{articles, favorites};
use crate::middleware::ContextExt;
use crate::web::articles::responses::ArticlesResponse;
use crate::web::diesel_error;
use crate::Repo;
use itertools::Itertools;
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

pub async fn feed(cx: Request<Repo>) -> tide::Result<Response> {
    // This can be avoided once https://github.com/http-rs/tide/pull/384 gets merged
    let query = cx.query::<FeedQuery>().unwrap_or(FeedQuery::default());

    let user_id = cx
        .get_claims()
        .map_err(|_| http::StatusCode::UNAUTHORIZED)?
        .user_id();
    let repo = cx.state();
    let result = articles::feed(repo, user_id, query.limit as i64, query.offset as i64)
        .map_err(|e| diesel_error(&e))?;

    let article_ids = result.iter().map(|(a, _, _)| a.id.to_owned()).collect_vec();
    let favs =
        favorites::are_favorite(&repo, user_id, article_ids).map_err(|e| diesel_error(&e))?;
    let result_with_fav = result
        .into_iter()
        .map(|(a, u, fav_count)| {
            let favorited = favs[&a.id];
            (a, u, fav_count, favorited)
        })
        .collect();

    let response = ArticlesResponse::new(result_with_fav);
    Ok(Response::new(200).body_json(&response).unwrap())
}
