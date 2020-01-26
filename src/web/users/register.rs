use super::responses::UserResponse;
use crate::domain::repositories::Repository;
use crate::domain::SignUp;
use crate::web::auth::encode_token;
use crate::Context;
use serde::Deserialize;
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

impl From<RegistrationRequest> for SignUp {
    fn from(r: RegistrationRequest) -> Self {
        Self {
            username: r.user.username,
            password: r.user.password,
            email: r.user.email,
        }
    }
}

pub async fn register<R: 'static + Repository + Sync + Send>(
    mut cx: Request<Context<R>>,
) -> Result<Response, Response> {
    let registration: RegistrationRequest = cx
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;
    let repository = &cx.state().repository;

    let sign_up: SignUp = registration.into();
    let new_user = repository.sign_up(sign_up)?;
    let token = encode_token(new_user.id);

    let response = UserResponse::from((new_user, token));
    Ok(Response::new(200).body_json(&response).unwrap())
}
