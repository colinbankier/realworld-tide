use crate::db::models;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    pub articles_count: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub article: Article,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub favorited: bool,
    pub favorites_count: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Author,
    pub tag_list: Vec<String>,
}

impl Article {
    pub fn new(a: models::Article, u: models::User, n_fav: i64, favorited: bool) -> Self {
        Self {
            title: a.title,
            slug: a.slug,
            description: a.description,
            body: a.body,
            favorited,
            favorites_count: n_fav as u64,
            created_at: a.created_at,
            updated_at: a.updated_at,
            tag_list: a.tag_list,
            author: Author {
                username: u.username,
                bio: u.bio,
                image: u.image,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl ArticlesResponse {
    pub fn new(results: Vec<(models::Article, models::User, i64, bool)>) -> Self {
        let articles_count = results.len() as u64;
        let articles = results
            .into_iter()
            .map(|(a, u, n_fav, favorited)| Article::new(a, u, n_fav, favorited))
            .collect();
        Self {
            articles,
            articles_count,
        }
    }
}
