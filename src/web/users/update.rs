use crate::conduit::users;
use crate::db::models::*;
use crate::middleware::ContextExt;
use crate::web::diesel_error;
use crate::Repo;
use log::info;
use serde::Deserialize;

use crate::web::users::responses::UserResponse;
use http::status::StatusCode;
use tide::{Request, Response};

#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    user: UpdateUser,
}

pub async fn update_user(mut cx: Request<Repo>) -> tide::Result<Response> {
    let update_params: UpdateUserRequest =
        cx.body_json().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let auth = cx.get_claims().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let repo = cx.state();
    info!("Update user {} {:?}", auth.user_id(), update_params);
    let results = users::update(repo, auth.user_id(), update_params.user);

    match results {
        Ok(user) => Ok(Response::new(200)
            .body_json(&UserResponse::new(user))
            .unwrap()),
        Err(e) => Err(diesel_error(&e)),
    }
}
