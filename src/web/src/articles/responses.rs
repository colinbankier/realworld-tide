use chrono::{DateTime, Utc};
use domain;
use domain::Profile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    pub articles_count: u64,
}

impl<T: Into<Article>> From<Vec<T>> for ArticlesResponse {
    fn from(articles: Vec<T>) -> Self {
        let articles_count = articles.len() as u64;
        let articles = articles.into_iter().map(|a| a.into()).collect();
        Self {
            articles,
            articles_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub article: Article,
}

impl<T: Into<Article>> From<T> for ArticleResponse {
    fn from(a: T) -> Self {
        Self { article: a.into() }
    }
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

impl From<domain::Article> for Article {
    fn from(a: domain::Article) -> Self {
        Self {
            title: a.content.title,
            slug: a.slug,
            description: a.content.description,
            body: a.content.body,
            tag_list: a.content.tag_list,
            favorited: false,
            favorites_count: a.favorites_count,
            created_at: a.metadata.created_at,
            updated_at: a.metadata.updated_at,
            author: a.author.into(),
        }
    }
}

impl From<domain::ArticleView> for Article {
    fn from(a: domain::ArticleView) -> Self {
        Self {
            title: a.content.title,
            slug: a.slug,
            description: a.content.description,
            body: a.content.body,
            tag_list: a.content.tag_list,
            favorited: a.favorited,
            favorites_count: a.favorites_count,
            created_at: a.metadata.created_at,
            updated_at: a.metadata.updated_at,
            author: a.author.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<domain::Profile> for Author {
    fn from(p: Profile) -> Self {
        Self {
            following: false,
            username: p.username,
            bio: p.bio,
            image: p.image,
        }
    }
}

impl From<domain::ProfileView> for Author {
    fn from(p: domain::ProfileView) -> Self {
        Self {
            username: p.profile.username,
            bio: p.profile.bio,
            image: p.profile.image,
            following: p.following,
        }
    }
}
