use super::responses::UserResponse;
use crate::conduit::users;
use crate::db::models::*;
use crate::web::diesel_error;
use crate::Repo;
use serde::Deserialize;

use tide::{Request, Response};

#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    user: UpdateUser,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    user: AuthUser,
}

#[derive(Deserialize)]
pub struct AuthUser {
    email: String,
    password: String,
}

pub async fn login(mut cx: Request<Repo>) -> Result<Response, Response> {
    let auth: AuthRequest = cx.body_json().await.map_err(|_| Response::new(400))?;
    let repo = cx.state();
    let user = auth.user;
    let result = users::find_by_email_password(repo, user.email, user.password);

    match result {
        Ok(user) => Ok(Response::new(200)
            .body_json(&UserResponse::new(user))
            .unwrap()),
        Err(diesel::result::Error::NotFound) => Err(Response::new(401)),
        Err(e) => Err(diesel_error(&e)),
    }
}
