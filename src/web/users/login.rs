use super::responses::UserResponse;
use crate::db::models::*;
use crate::Repo;
use serde::Deserialize;

use crate::auth::encode_token;
use crate::domain::repositories::UsersRepository;
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
    let user = cx
        .body_json::<AuthRequest>()
        .await
        .map_err(|_| Response::new(400))?
        .user;
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let logged_in_user = repository.get_by_email_and_password(&user.email, &user.password)?;
    let token = encode_token(logged_in_user.id);

    let response = UserResponse::from((logged_in_user, token));

    Ok(Response::new(200).body_json(&response).unwrap())
}
