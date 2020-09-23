use crate::{Context, ErrorResponse};
use domain::repositories::Repository;
use serde::{Deserialize, Serialize};
use tide::prelude::*;
use tide::{Request, Response};

#[derive(Serialize, Deserialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

pub async fn tags<R: 'static + Repository + Sync + Send>(
    cx: Request<Context<R>>,
) -> Result<Response, ErrorResponse> {
    let repository = &cx.state().repository;
    let tags = repository.get_tags()?;
    let response = TagsResponse {
        tags: tags.into_iter().collect(),
    };
    Ok(Response::builder(200).body(json!(&response)).into())
}
