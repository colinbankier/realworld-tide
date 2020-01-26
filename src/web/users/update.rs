use crate::domain;
use crate::middleware::ContextExt;
use crate::Repo;
use serde::{Deserialize, Serialize};

use crate::auth::encode_token;
use crate::domain::repositories::UsersRepository;
use crate::web::users::responses::UserResponse;
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

impl From<UpdateUserRequest> for domain::UserUpdate {
    fn from(u: UpdateUserRequest) -> Self {
        Self {
            email: u.email,
            username: u.username,
            password: u.password,
            image: u.image,
            bio: u.bio,
        }
    }
}

pub async fn update_user(mut cx: tide::Request<Repo>) -> Result<Response, Response> {
    let update_params = cx
        .body_json::<Request>()
        .await
        .map_err(|_| Response::new(400))?
        .user;
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let user = repository.get_by_id(user_id)?;
    let updated_user = user.update(update_params.into(), &repository)?;
    let token = encode_token(updated_user.id);

    let response = UserResponse::from((updated_user, token));

    Ok(Response::new(200).body_json(&response).unwrap())
}
