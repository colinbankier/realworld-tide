use crate::middleware::ContextExt;
use crate::profiles::responses::ProfileResponse;
use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use tide::prelude::*;
use tide::{Request, Response};
use uuid::Uuid;

pub async fn get_profile<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let profile_username: String = cx.param("username").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    let response: ProfileResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_user_by_id(user_id)?;
            let view = repository.get_profile_view(&user, &profile_username)?;
            ProfileResponse::from(view)
        }
        None => {
            let profile = repository.get_profile(&profile_username)?;
            ProfileResponse::from(profile)
        }
    };

    Ok(Response::builder(200).body(json!(&response)).into())
}
