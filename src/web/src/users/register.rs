use super::responses::UserResponse;
use crate::auth::encode_token;
use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use domain::SignUp;
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
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

impl TryFrom<RegistrationRequest> for SignUp {
    type Error = domain::PasswordError;

    fn try_from(r: RegistrationRequest) -> Result<Self, Self::Error> {
        let sign_up = Self {
            username: r.user.username,
            password: domain::Password::from_clear_text(r.user.password)?,
            email: r.user.email,
        };
        Ok(sign_up)
    }
}

pub async fn register<R: 'static + Repository + Sync + Send>(
    mut cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let registration: RegistrationRequest = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let repository = &cx.state().repository;

    let sign_up: SignUp = registration.try_into()?;
    let new_user = repository.sign_up(sign_up)?;
    let token = encode_token(new_user.id);

    let response = UserResponse::from((new_user, token));
    Ok(Response::new(200).body_json(&response).unwrap())
}
