use crate::middleware::ContextExt;
use crate::Repo;
use log::info;

use crate::domain::repositories::UsersRepository;
use crate::web::users::responses::UserResponse;
use tide::{Request, Response};

pub async fn get_current_user(cx: Request<Repo>) -> Result<Response, Response> {
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = crate::conduit::articles_repository::Repository(cx.state());
    info!("Get user {}", user_id);

    let user = repository.get_by_id(user_id)?;

    let payload: UserResponse = user.into();
    let response = Response::new(200).body_json(&payload).unwrap();
    Ok(response)
}
