use crate::db::Repo;
use crate::models::*;
use diesel::prelude::*;
use http::status::StatusCode;
use tide::{self, body::Json, AppData};
use jsonwebtoken::{encode, Algorithm, Header};
use crate::auth;
use crate::models::User;

#[derive(Deserialize, Debug)]
pub struct Registration {
    user: NewUser,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUser {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    image: Option<String>,
    bio: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    user: User,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    user: AuthUser,
}

#[derive(Deserialize)]
pub struct AuthUser {
    email: String,
    password: String,
}

pub async fn register(
    repo: AppData<Repo>,
    registration: Json<Registration>,
) -> Result<Json<UserResponse>, StatusCode> {
    use crate::schema::users;

    let result = await! { repo.run(|conn| {
        let user = registration.0.user;
        // TODO: store password not in plain text, later
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn)
    })
    };

    result
        .map(|user| Json(UserResponse { user }))
        .map_err(|e| diesel_error(&e))
}

pub async fn login(
    repo: AppData<Repo>,
    auth: Json<AuthRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    use crate::schema::users::dsl::*;

    let user = auth.0.user;
    let result = await! { repo.run(|conn| {
    users
        .filter(email.eq(user.email))
        .filter(password.eq(user.password))
        .first::<User>(&conn)
    }) };

    match result {
        Ok(user) => {
            let user = User{token: Some(auth::encode_token(user.id)), ..user};
            Ok(Json(UserResponse { user }))
        },
        Err(diesel::result::Error::NotFound) => Err(StatusCode::UNAUTHORIZED),
        Err(e) => Err(diesel_error(&e))
    }
}

pub async fn get_user(repo: AppData<Repo>) -> Result<Json<UserResponse>, StatusCode> {
    use crate::schema::users::dsl::*;

    let results = await! { repo.run(|conn| users.first::<User>(&conn)) };

    results
        .map(|user| Json(UserResponse { user }))
        .map_err(|e| diesel_error(&e))
}

// pub async fn update_user(
//     repo: AppData<Repo>,
//     update_user: Json<UpdateUser>,
// ) -> Result<Json<UserResponse>, StatusCode> {
// }

pub async fn list_articles(repo: AppData<Repo>) -> Result<Json<Vec<Article>>, StatusCode> {
    use crate::schema::articles::dsl::*;

    let results = await! { repo.run(|conn| articles.limit(10).load::<Article>(&conn)) };

    results
        .map(|article_list| Json(article_list))
        .map_err(|e| diesel_error(&e))
}

fn diesel_error(e: &diesel::result::Error) -> StatusCode {
    error!("{}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}
