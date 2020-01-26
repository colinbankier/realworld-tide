use crate::domain::repositories::Repository;
use crate::Context;
use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[derive(Serialize, Deserialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

pub async fn tags<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, Response> {
    let repository = &cx.state().repository;
    let tags = repository.get_tags()?;
    let response = TagsResponse {
        tags: tags.into_iter().collect(),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
