use crate::conduit::followers;
use crate::conduit::users;
use crate::middleware::ContextExt;
use crate::web::internal_error;
use crate::Repo;

use crate::web::profiles::responses::ProfileResponse;
use tide::{Request, Response, ResultExt};
use uuid::Uuid;

pub async fn get_profile(cx: Request<Repo>) -> tide::Result<Response> {
    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let profile_username: String = cx.param("username").client_err()?;
    let repo = cx.state();
    let profile =
        users::find_by_username(repo, profile_username).map_err(|e| internal_error(&e))?;
    let is_following = match user_id {
        Some(user_id) => {
            followers::is_following(&repo, user_id, profile.id).map_err(|e| internal_error(&e))?
        }
        None => false,
    };
    Ok(Response::new(200)
        .body_json(&ProfileResponse::new(profile, is_following))
        .unwrap())
}
