use crate::conduit::{articles, articles::ArticleQuery, favorites};
use crate::db::models::{Article, User};
use crate::middleware::ContextExt;
use crate::web::articles::responses::ArticlesResponse;
use crate::web::diesel_error;
use crate::Repo;
use itertools::Itertools;
use tide::{Request, Response};
use uuid::Uuid;

pub async fn list_articles(cx: Request<Repo>) -> tide::Result<Response> {
    // This can be avoided once https://github.com/http-rs/tide/pull/384 gets merged
    let query = cx.query::<ArticleQuery>().unwrap_or(ArticleQuery {
        author: None,
        favorited: None,
        tag: None,
    });

    let repo = cx.state();
    let result = articles::find(repo, query).map_err(|e| diesel_error(&e))?;

    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let result_with_fav: Vec<(Article, User, i64, bool)> = match user_id {
        // If we are logged in, check for each article if the user marked it as favorite
        Some(user_id) => {
            let article_ids = result.iter().map(|(a, _, _)| a.id.to_owned()).collect_vec();
            let favs = favorites::are_favorite(&repo, user_id, article_ids)
                .map_err(|e| diesel_error(&e))?;
            result
                .into_iter()
                .map(|(a, u, fav_count)| {
                    let favorited = favs[&a.id];
                    (a, u, fav_count, favorited)
                })
                .collect()
        }
        // If we are not logged in, mark everything as not favorited
        None => result
            .into_iter()
            .map(|(a, u, fav_count)| (a, u, fav_count, false))
            .collect(),
    };

    let response = ArticlesResponse::new(result_with_fav);
    Ok(Response::new(200).body_json(&response).unwrap())
}
