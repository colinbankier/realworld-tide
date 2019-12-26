use crate::db::models;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    pub articles_count: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ArticlesResponse {
    pub fn new(articles: Vec<models::Article>) -> Self {
        let articles_count = articles.len() as u64;
        let articles = articles
            .into_iter()
            .map(|a| Article {
                title: a.title,
                slug: a.slug,
                description: a.description,
                body: a.body,
                created_at: a.created_at,
                updated_at: a.updated_at,
            })
            .collect();
        Self {
            articles,
            articles_count,
        }
    }
}
