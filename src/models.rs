use crate::schema::users;
use crate::schema::articles;
use chrono::NaiveDateTime;
use diesel::result::Error;

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub user_id: i32,
}