use crate::middleware::JwtContext;
use crate::{Context, ErrorResponse};

use crate::profiles::responses::ProfileResponse;
use domain::repositories::Repository;
use tide::{Request, Response};

pub enum Action {
    Follow,
    Unfollow,
}

pub async fn follow<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    _follow(cx, Action::Follow).await
}

pub async fn unfollow<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    _follow(cx, Action::Unfollow).await
}

async fn _follow<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
    action: Action,
) -> Result<Response, ErrorResponse> {
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let profile_username: String = cx.param("username").map_err(|_| Response::new(400))?;
    let repository = &cx.state().repository;

    let user = repository.get_user_by_id(user_id)?;
    let profile = repository.get_profile(&profile_username)?;
    let view = match action {
        Action::Follow => user.follow(profile, repository)?,
        Action::Unfollow => user.unfollow(profile, repository)?,
    };

    let response = ProfileResponse::from(view);
    Ok(Response::new(200).body_json(&response).unwrap())
}
