use crate::conduit::followers;
use crate::conduit::users;
use crate::middleware::ContextExt;
use crate::web::diesel_error;
use crate::Repo;

use crate::web::profiles::responses::ProfileResponse;
use http::StatusCode;
use tide::{Request, Response, ResultExt};

pub async fn follow(cx: Request<Repo>) -> tide::Result<Response> {
    let user_id = cx
        .get_claims()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .user_id();
    let profile_username: String = cx.param("username").client_err()?;
    let repo = cx.state();
    let profile = users::find_by_username(repo, profile_username).map_err(|e| diesel_error(&e))?;
    followers::follow(&repo, user_id, profile.id).map_err(|e| diesel_error(&e))?;
    Ok(Response::new(200)
        .body_json(&ProfileResponse::new(profile, true))
        .unwrap())
}

pub async fn unfollow(cx: Request<Repo>) -> tide::Result<Response> {
    let user_id = cx
        .get_claims()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .user_id();
    let profile_username: String = cx.param("username").client_err()?;
    let repo = cx.state();
    let profile = users::find_by_username(repo, profile_username).map_err(|e| diesel_error(&e))?;
    followers::unfollow(&repo, user_id, profile.id).map_err(|e| diesel_error(&e))?;
    Ok(Response::new(200)
        .body_json(&ProfileResponse::new(profile, false))
        .unwrap())
}
