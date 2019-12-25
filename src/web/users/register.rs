use crate::auth::encode_token;
use crate::conduit::users;
use crate::models::*;
use crate::web::diesel_error;
use crate::Repo;
use serde::Deserialize;
use uuid::Uuid;

use http::status::StatusCode;
use tide::{Request, Response};

#[derive(Deserialize, Debug)]
pub struct RegistrationRequest {
    user: NewUserRequest,
}

#[derive(Deserialize, Debug)]
pub struct NewUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(mut cx: Request<Repo>) -> tide::Result<Response> {
    let registration: RegistrationRequest =
        cx.body_json().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let repo = cx.state();

    let user_id = Uuid::new_v4();
    let token = encode_token(user_id);

    let new_user = NewUser {
        username: registration.user.username,
        email: registration.user.email,
        password: registration.user.password,
        token,
        id: user_id,
    };
    let result = users::insert(repo, new_user);

    result
        .map(|b| Response::new(200).body_json(&b).unwrap())
        .map_err(|e| diesel_error(&e))
}
