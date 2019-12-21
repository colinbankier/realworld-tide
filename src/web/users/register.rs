use super::responses::UserResponse;
use crate::conduit::users;
use crate::db::models::*;
use crate::web::internal_error;
use crate::Repo;
use serde::Deserialize;
use tide::{Request, Response};
use uuid::Uuid;

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
    let registration: RegistrationRequest = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let repo = cx.state();

    let new_user = NewUser {
        username: registration.user.username,
        email: registration.user.email,
        password: registration.user.password,
        id: Uuid::new_v4(),
    };
    let result = users::insert(repo, new_user);

    result
        .map(|user| {
            Response::new(200)
                .body_json(&UserResponse::new(user))
                .unwrap()
        })
        .map_err(|e| internal_error(&e))
}
