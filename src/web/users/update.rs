use crate::conduit::users;
use crate::db::models::*;
use crate::middleware::ContextExt;
use crate::web::diesel_error;
use crate::Repo;
use log::info;
use serde::{Deserialize, Serialize};

use crate::web::users::responses::UserResponse;
use tide::{Request, Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub user: UpdateUser,
}

pub async fn update_user(mut cx: Request<Repo>) -> Result<Response, Response> {
    let update_params: UpdateUserRequest = cx.body_json().await.map_err(|_| Response::new(400))?;
    let auth = cx.get_claims().map_err(|_| Response::new(401))?;
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
