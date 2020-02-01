use super::responses::UserResponse;
use crate::{Context, ErrorResponse};
use serde::Deserialize;

use crate::auth::encode_token;
use domain::repositories::Repository;
use tide::{Request, Response};

#[derive(Deserialize)]
pub struct AuthRequest {
    user: AuthUser,
}

#[derive(Deserialize)]
pub struct AuthUser {
    email: String,
    password: String,
}

pub async fn login<R: 'static + Repository + Sync + Send>(
    mut cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let user = cx
        .body_json::<AuthRequest>()
        .await
        .map_err(|_| Response::new(400))?
        .user;
    let repository = &cx.state().repository;

    let logged_in_user = repository.get_user_by_email_and_password(&user.email, &user.password)?;
    let token = encode_token(logged_in_user.id);

    let response = UserResponse::from((logged_in_user, token));

    Ok(Response::new(200).body_json(&response).unwrap())
}
