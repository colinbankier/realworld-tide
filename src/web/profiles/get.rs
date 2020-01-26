use crate::domain::repositories::UsersRepository;
use crate::middleware::ContextExt;
use crate::web::profiles::responses::ProfileResponse;
use crate::Repo;
use tide::{Request, Response};
use uuid::Uuid;

pub async fn get_profile(cx: Request<Repo>) -> Result<Response, Response> {
    let user_id: Option<Uuid> = cx.get_claims().map(|c| c.user_id()).ok();
    let profile_username: String = cx.param("username").map_err(|_| Response::new(400))?;
    let repository = crate::conduit::articles_repository::Repository(cx.state());

    let response: ProfileResponse = match user_id {
        Some(user_id) => {
            let user = repository.get_by_id(user_id)?;
            let view = repository.get_view(&user, &profile_username)?;
            ProfileResponse::from(view)
        }
        None => {
            let profile = repository.get_profile(&profile_username)?;
            ProfileResponse::from(profile)
        }
    };

    Ok(Response::new(200).body_json(&response).unwrap())
}
