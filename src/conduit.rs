use crate::db::ConnectionPool;
use crate::models::*;
use diesel::prelude::*;
use http::status::StatusCode;
use tide::{self, body::Json, AppData};

pub async fn get_user(pool: AppData<ConnectionPool>) -> Result<Json<User>, StatusCode> {
    use crate::schema::users::dsl::*;

    let connection = pool.get().unwrap();
    let results = users.first::<User>(&connection);

    results
        .map(|user| Json(user))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn list_articles(
    pool: AppData<ConnectionPool>,
) -> Result<Json<Vec<Article>>, StatusCode> {
    use crate::schema::articles::dsl::*;

    let connection = pool.get().unwrap();
    let results = articles.limit(10).load::<Article>(&connection);

    results
        .map(|article_list| Json(article_list))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
