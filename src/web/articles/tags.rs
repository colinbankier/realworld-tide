use crate::conduit::articles;
use crate::web::internal_error;
use crate::Repo;
use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[derive(Serialize, Deserialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

pub async fn tags(cx: Request<Repo>) -> tide::Result<Response> {
    let repo = cx.state();
    let tags = articles::tags(repo).map_err(|e| internal_error(&e))?;

    let response = TagsResponse {
        tags: tags.into_iter().collect(),
    };
    Ok(Response::new(200).body_json(&response).unwrap())
}
