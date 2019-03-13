use crate::auth::{encode_token, Claims};
use crate::conduit::users;
use crate::db::Repo;
use crate::models::*;
use crate::web::diesel_error;
use serde_derive::{Deserialize, Serialize};
use log::info;

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
    use crate::test_helpers::test_server::{response_json, TestServer};
    use crate::Repo;
    use http::Request;
    use http_service::Body;
    use serde_json::{json, Value};
    use tokio_async_await_test::async_test;

    #[async_test]
    async fn register_and_login() {
        let server = TestServer::new(Repo::new());
        let user = generate::new_user();

        await! {register_user(&server, &user)};
        let token = await! {login_user(&server, &user)};
        let user_details = await! { get_user_details(&server, &token)};

        assert_eq!(user_details["user"]["username"], user.username);
        assert_eq!(user_details["user"]["email"], user.email);
    }

    #[async_test]
    async fn update_and_retrieve_user_details() {
        let server = TestServer::new(Repo::new());
        let user = generate::new_user();

        let stored_user = await! {register_user(&server, &user)};
        let token = await! {login_user(&server, &user)};

        assert_eq!(stored_user["user"]["bio"], Value::Null);
        assert_eq!(stored_user["user"]["image"], Value::Null);

        let new_details = json!({
            "user": {
                "bio": "I like to code.",
                "image": "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
            }
        });
        let updated_user = await! { update_user_details(&server, &new_details, &token)};
        assert_eq!(updated_user["user"]["bio"], new_details["user"]["bio"]);
        assert_eq!(updated_user["user"]["image"], new_details["user"]["image"]);

        let user_details = await! { get_user_details(&server, &token)};
        assert_eq!(user_details["user"]["bio"], new_details["user"]["bio"]);
        assert_eq!(user_details["user"]["image"], new_details["user"]["image"]);
    }

    async fn register_user<'a>(server: &'a TestServer, user: &'a NewUser) -> Value {
        let res = await!(server.call(
            Request::post("/api/users")
                .body(
                    json!({
                        "user": {
                            "email": user.email,
                            "password": user.password,
                            "username": user.username,
                        }
                    })
                    .to_string()
                    .into()
                )
                .unwrap()
        ));
        await!(response_json(res))
    }

    async fn login_user<'a>(server: &'a TestServer, user: &'a NewUser) -> String {
        let res = await!(server.call(
            Request::post("/api/users/login")
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
                .unwrap()
        ));
        assert_eq!(res.status(), 200);

        let response_json = await!(response_json(res));

        assert!(response_json["user"]["token"].is_string());
        response_json["user"]["token"]
            .as_str()
            .expect("Token not found")
            .to_string()
    }

    async fn get_user_details<'a>(server: &'a TestServer, token: &'a String) -> Value {
        let res = await!(server.call(
            Request::get("/api/user")
                .header("Authorization", format!("token: {}", token))
                .body(Body::empty())
                .unwrap()
        ));
        assert_eq!(res.status(), 200);
        await!(response_json(res))
    }

    async fn update_user_details<'a>(
        server: &'a TestServer,
        details: &'a Value,
        token: &'a String,
    ) -> Value {
        let res = await!(server.call(
            Request::put("/api/user")
                .header("Authorization", format!("token: {}", token))
                .body(details.to_string().into())
                .unwrap()
        ));
        assert_eq!(res.status(), 200);
        await!(response_json(res))
    }
}
