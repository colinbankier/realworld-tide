use super::responses::UserResponse;
use crate::auth::encode_token;
use crate::conduit::users;
use crate::models::*;
use crate::web::diesel_error;
use crate::Repo;
use serde::Deserialize;

use http::status::StatusCode;
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

pub async fn login(mut cx: Request<Repo>) -> tide::Result<Response> {
    let auth: AuthRequest = cx.body_json().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let repo = cx.state();
    let user = auth.user;
    let result = users::find_by_email_password(repo, user.email, user.password);

    match result {
        Ok(user) => {
            let user = User {
                token: Some(encode_token(user.id)),
                ..user
            };
            let response = UserResponse { user };
            Ok(Response::new(200).body_json(&response).unwrap())
        }
        Err(diesel::result::Error::NotFound) => Err(StatusCode::UNAUTHORIZED.into()),
        Err(e) => Err(diesel_error(&e)),
    }
}
