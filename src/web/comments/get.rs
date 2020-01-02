use crate::conduit::{articles, comments};
use crate::middleware::ContextExt;
use crate::web::comments::responses::{Comment, CommentsResponse};
use crate::web::diesel_error;
use crate::Repo;
use tide::{Response, ResultExt};
use uuid::Uuid;

pub async fn get(cx: tide::Request<Repo>) -> tide::Result<Response> {
    let _user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let slug: String = cx.param("slug").client_err()?;
    let repo = cx.state();

    let (article, _, _) = articles::find_one(&repo, &slug).map_err(|e| diesel_error(&e))?;

    let results = comments::get_comments(repo, article.id).map_err(|e| diesel_error(&e))?;
    let response = CommentsResponse {
        comments: results
            .into_iter()
            .map(|(c, a)| Comment::new(c, a))
            .collect(),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
