use crate::auth::encode_token;
use crate::conduit::users;
use crate::models::*;
use crate::web::diesel_error;
use crate::Repo;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Debug)]
pub struct NewUserResponse {
    pub user: User,
}

pub async fn register(mut cx: Request<Repo>) -> tide::Result<Response> {
    let registration: RegistrationRequest = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
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
        .map(|user| {
            Response::new(200)
                .body_json(&NewUserResponse { user })
                .unwrap()
        })
        .map_err(|e| diesel_error(&e))
}
