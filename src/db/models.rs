use crate::db::schema::articles;
use crate::db::schema::comments;
use crate::db::schema::favorites;
use crate::db::schema::followers;
use crate::db::schema::users;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub user_id: Uuid,
}

#[derive(AsChangeset, Deserialize, Debug, Clone)]
#[table_name = "articles"]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "favorites"]
pub struct NewFavorite {
    pub user_id: Uuid,
    pub article_id: i32,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "followers"]
pub struct NewFollower {
    pub followed_id: Uuid,
    pub follower_id: Uuid,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "comments"]
pub struct NewComment {
    pub author_id: Uuid,
    pub article_id: i32,
    pub body: String,
}

#[derive(Queryable, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: i64,
    pub author_id: Uuid,
    pub article_id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
