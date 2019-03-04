use jsonwebtoken::{encode, Algorithm, Header};

use crate::auth::{encode_token, Claims};
use crate::conduit::users;
use crate::db::Repo;
use crate::models::*;
use crate::web::diesel_error;

use http::status::StatusCode;
use tide::{self, body::Json, AppData};

#[derive(Deserialize, Debug)]
pub struct Registration {
    user: NewUser,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    user: UpdateUser,
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
    let result = await! { users::insert(repo.clone(), registration.0.user) };

    result
        .map(|user| Json(UserResponse { user }))
        .map_err(|e| diesel_error(&e))
}

pub async fn login(
    repo: AppData<Repo>,
    auth: Json<AuthRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = auth.0.user;
    let result = await! {
        users::find_by_email_password(repo.clone(), user.email, user.password)
    };

    match result {
        Ok(user) => {
            let user = User {
                token: Some(encode_token(user.id)),
                ..user
            };
            Ok(Json(UserResponse { user }))
        }
        Err(diesel::result::Error::NotFound) => Err(StatusCode::UNAUTHORIZED),
        Err(e) => Err(diesel_error(&e)),
    }
}

pub async fn get_user(repo: AppData<Repo>, auth: Claims) -> Result<Json<UserResponse>, StatusCode> {
    info!("Get user {}", auth.user_id());

    let results = await! { users::find(repo.clone(), auth.user_id()) };

    results
        .map(|user| Json(UserResponse { user }))
        .map_err(|e| diesel_error(&e))
}

pub async fn update_user(
    repo: AppData<Repo>,
    update_params: Json<UpdateUserRequest>,
    auth: Claims,
) -> Result<Json<UserResponse>, StatusCode> {
    info!("Update user {} {:?}", auth.user_id(), update_params.0);
    let results = await! {
        users::update(repo.clone(), auth.user_id(), update_params.0.user)
    };

    results
        .map(|user| Json(UserResponse { user }))
        .map_err(|e| diesel_error(&e))
}

#[cfg(test)]
mod tests {
    use crate::models::NewUser;
    use crate::test_helpers::generate;
    use crate::test_helpers::test_server::{self, TestServer};
    use crate::Repo;
    use http_service::Body;
    use serde_json::{json, Value};
    use std::str::from_utf8;
    use tokio_async_await_test::async_test;

    #[async_test]
    async fn register_and_login() {
        let server = test_server::new(Repo::new());
        let user = generate::new_user();

        await! {register_user(&server, &user)};
        let token = await! {login_user(&server, &user)};
        let user_details = await! { get_user_details(&server, token)};

        assert_eq!(user_details["user"]["username"], user.username);
        assert_eq!(user_details["user"]["email"], user.email);
    }

    async fn register_user<'a>(server: &'a TestServer, user: &'a NewUser) {
        let req = http::Request::post("/api/users")
            .body(
                json!({
                    "user": {
                        "email": user.email,
                        "password": user.password,
                        "username": user.username,
                    }
                })
                .to_string()
                .into(),
            )
            .unwrap();
        let res = await! { server.simulate(req) };
        assert_eq!(res.status(), 200);
    }

    async fn login_user<'a>(server: &'a TestServer, user: &'a NewUser) -> String {
        let req = http::Request::post("/api/users/login")
            .body(
                json!({
                    "user": {
                        "email": user.email,
                        "password": user.password,
                    }
                })
                .to_string()
                .into(),
            )
            .unwrap();
        let res = await! { server.simulate(req) };
        assert_eq!(res.status(), 200);
        let body = await! { res.into_body().into_vec() }.unwrap();

        let response_json: Value =
            serde_json::from_str(from_utf8(&body).unwrap()).expect("Could not parse body.");
        assert!(response_json["user"]["token"].is_string());

        response_json["user"]["token"]
            .as_str()
            .expect("Token not found")
            .to_string()
    }

    async fn get_user_details<'a>(server: &'a TestServer, token: String) -> Value {
        let req = http::Request::get("/api/user")
            .header("Authorization", format!("token: {}", token))
            .body(Body::empty())
            .unwrap();
        let res = await! { server.simulate(req) };
        assert_eq!(res.status(), 200);
        let body = await! { res.into_body().into_vec() }.unwrap();

        serde_json::from_str(from_utf8(&body).unwrap()).expect("Could not parse body.")
    }
}
