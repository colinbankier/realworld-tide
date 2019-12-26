use crate::db::models::Article;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub articles: Vec<Article>,
    pub articles_count: u64,
}

impl ArticleResponse {
    pub fn new(articles: Vec<Article>) -> Self {
        let articles_count = articles.len() as u64;
        Self {
            articles,
            articles_count,
        }
    }
}
