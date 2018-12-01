use crate::db::Repo;
use crate::models::*;
use diesel::prelude::*;
use http::status::StatusCode;
use tide::{self, body::Json, AppData};

pub async fn get_user(repo: AppData<Repo>) -> Result<Json<User>, StatusCode> {
    use crate::schema::users::dsl::*;

    let results = await! { repo.run(|connection| users.first::<User>(&connection)) };

    match results {
        Ok(diesel_result) => diesel_result
            .map(|user| Json(user))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_articles(repo: AppData<Repo>) -> Result<Json<Vec<Article>>, StatusCode> {
    use crate::schema::articles::dsl::*;

    let results = await! { repo.run(|connection| articles.limit(10).load::<Article>(&connection)) };

    match results {
        Ok(diesel_result) => diesel_result
            .map(|article_list| Json(article_list))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
