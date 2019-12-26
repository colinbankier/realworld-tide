use crate::db::models::Article;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub articles: Vec<Article>,
    pub articles_count: u64,
}
