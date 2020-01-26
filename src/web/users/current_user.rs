use crate::web::middleware::ContextExt;
use crate::Context;
use log::info;

use crate::domain::repositories::Repository;
use crate::web::auth::encode_token;
use crate::web::users::responses::UserResponse;
use tide::{Request, Response};

pub async fn get_current_user<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, Response> {
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let repository = &cx.state().repository;
    info!("Get user {}", user_id);

    let user = repository.get_user_by_id(user_id)?;
    let token = encode_token(user.id);

    let payload: UserResponse = (user, token).into();
    let response = Response::new(200).body_json(&payload).unwrap();
    Ok(response)
}
