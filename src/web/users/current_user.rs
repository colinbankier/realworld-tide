use crate::conduit::users;
use crate::middleware::ContextExt;
use crate::Repo;
use log::info;

use crate::web::users::responses::UserResponse;
use tide::{Request, Response};

pub async fn get_current_user(cx: Request<Repo>) -> Result<Response, Response> {
    let auth = cx.get_claims().map_err(|_| Response::new(401))?;
    let repo = cx.state();
    info!("Get user {}", auth.user_id());

    let user = users::find(repo, auth.user_id())?;
    let response = Response::new(200)
        .body_json(&UserResponse::new(user))
        .unwrap();
    Ok(response)
}
