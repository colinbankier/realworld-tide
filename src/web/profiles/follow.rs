use crate::middleware::ContextExt;
use crate::Repo;

use crate::domain::repositories::UsersRepository;
use crate::web::profiles::responses::ProfileResponse;
use tide::{Request, Response};

pub enum Action {
    Follow,
    Unfollow,
}

pub async fn follow(cx: Request<Repo>) -> Result<Response, Response> {
    _follow(cx, Action::Follow).await
}

pub async fn unfollow(cx: Request<Repo>) -> Result<Response, Response> {
    _follow(cx, Action::Unfollow).await
}

async fn _follow(cx: Request<Repo>, action: Action) -> Result<Response, Response> {
    let user_id = cx.get_claims().map_err(|_| Response::new(401))?.user_id();
    let profile_username: String = cx.param("username").map_err(|_| Response::new(400))?;
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let user = repository.get_by_id(user_id)?;
    let profile = repository.get_profile(&profile_username)?;
    let view = match action {
        Action::Follow => user.follow(profile, &repository)?,
        Action::Unfollow => user.unfollow(profile, &repository)?,
    };

    let response = ProfileResponse::from(view);
    Ok(Response::new(200).body_json(&response).unwrap())
}
