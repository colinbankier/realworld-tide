use crate::middleware::ContextExt;
use crate::{Context, ErrorResponse};
use domain;
use serde::{Deserialize, Serialize};

use crate::auth::encode_token;
use crate::users::responses::UserResponse;
use domain::repositories::Repository;
use std::convert::{TryFrom, TryInto};
use tide::Response;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub user: UpdateUserRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

impl TryFrom<UpdateUserRequest> for domain::UserUpdate {
    type Error = domain::PasswordError;

    fn try_from(u: UpdateUserRequest) -> Result<Self, Self::Error> {
        let update = Self {
            email: u.email,
            username: u.username,
            password: u
                .password
                .map(|p| domain::Password::from_clear_text(p))
                .transpose()?,
            image: u.image,
            bio: u.bio,
        };
        Ok(update)
    }
}

pub async fn update_user<R: 'static + Repository + Sync + Send>(
    mut cx: tide::Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let update_params = cx
        .body_json::<Request>()
        .await
        .map_err(|_| Response::new(400))?
        .user;
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = &cx.state().repository;

    let user = repository.get_user_by_id(user_id)?;
    let updated_user = user.update(update_params.try_into()?, repository)?;
    let token = encode_token(updated_user.id);

    let response = UserResponse::from((updated_user, token));

    Ok(Response::new(200).body_json(&response).unwrap())
}
