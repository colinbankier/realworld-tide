use crate::db::Repo;
use crate::models::*;
use diesel::prelude::*;
use http::status::StatusCode;
use tide::{self, body::Json, AppData};

#[derive(Deserialize, Debug)]
pub struct Registration {
    user: NewUser,
}

#[derive(Serialize)]
pub struct UserResponse {
    user: User,
}

pub async fn register(
    repo: AppData<Repo>,
    registration: Json<Registration>,
) -> Result<Json<UserResponse>, StatusCode> {
    use crate::schema::users;

    let result = await! { repo.run(|conn| {
        let user = registration.0.user;
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
    })
    };

    match result {
        Ok(diesel_result) => diesel_result
            .map(|user| Json(UserResponse { user }))
            .map_err(|e| {
                error!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }),
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user(repo: AppData<Repo>) -> Result<Json<User>, StatusCode> {
    use crate::schema::users::dsl::*;

    let results = await! { repo.run(|conn| users.first::<User>(&conn)) };

    match results {
        Ok(diesel_result) => diesel_result
            .map(|user| Json(user))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_articles(repo: AppData<Repo>) -> Result<Json<Vec<Article>>, StatusCode> {
    use crate::schema::articles::dsl::*;

    let results = await! { repo.run(|conn| articles.limit(10).load::<Article>(&conn)) };

    match results {
        Ok(diesel_result) => diesel_result
            .map(|article_list| Json(article_list))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
