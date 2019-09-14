use crate::auth::encode_token;
use crate::conduit::users;
use crate::middleware::ContextExt;
use crate::models::*;
use crate::web::diesel_error;
use crate::Repo;
use log::info;
use serde_derive::{Deserialize, Serialize};

use http::status::StatusCode;
use tide::{response, Context, EndpointResult};

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

pub async fn register(mut cx: Context<Repo>) -> EndpointResult {
    let registration: Registration = cx.body_json().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let repo = cx.state();
    let result = users::insert(repo, registration.user).await;

    result
        .map(|user| response::json(user))
        .map_err(|e| diesel_error(&e))
}

pub async fn login(mut cx: Context<Repo>) -> EndpointResult {
    let auth: AuthRequest = cx.body_json().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let repo = cx.state();
    let user = auth.user;
    let result = users::find_by_email_password(repo.clone(), user.email, user.password).await;

    match result {
        Ok(user) => {
            let user = User {
                token: Some(encode_token(user.id)),
                ..user
            };
            Ok(response::json(user))
        }
        Err(diesel::result::Error::NotFound) => Err(StatusCode::UNAUTHORIZED.into()),
        Err(e) => Err(diesel_error(&e)),
    }
}

pub async fn get_user(mut cx: Context<Repo>) -> EndpointResult {
    let auth = cx.get_claims().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let repo = cx.state();
    info!("Get user {}", auth.user_id());

    let results = users::find(repo, auth.user_id()).await;

    results
        .map(|user| response::json(user))
        .map_err(|e| diesel_error(&e))
}

pub async fn update_user(mut cx: Context<Repo>) -> EndpointResult {
    let update_params: UpdateUserRequest =
        cx.body_json().await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let auth = cx.get_claims().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let repo = cx.state();
    info!("Update user {} {:?}", auth.user_id(), update_params);
    let results = users::update(repo, auth.user_id(), update_params.user).await;

    results
        .map(|user| response::json(user))
        .map_err(|e| diesel_error(&e))
}

#[cfg(test)]
mod tests {
    use crate::models::NewUser;
    use crate::test_helpers::generate;
    use crate::test_helpers::test_server::{get_repo, response_json, TestServer};
    use crate::Repo;
    use diesel::PgConnection;
    use futures::executor::ThreadPool;
    use http::Request;
    use http_service::Body;
    use serde_json::{json, Value};

    #[test]
    fn register_and_login() {
        let runtime = ThreadPool::new().unwrap();
        runtime.spawn_ok(async move {
            let server = TestServer::new(get_repo());
            let user = generate::new_user();

            register_user(&server, &user).await;
            let token = login_user(&server, &user).await;
            let user_details = get_user_details(&server, &token).await;

            assert_eq!(user_details["user"]["username"], user.username);
            assert_eq!(user_details["user"]["email"], user.email);
        })
    }

    // #[async_test]
    // async fn register_and_login() {
    //     let server = TestServer::new(Repo::new());
    //     let user = generate::new_user();

    //     register_user(&server, &user).await;
    //     let token = login_user(&server, &user).await;
    //     let user_details =  get_user_details(&server, &token).await;

    //     assert_eq!(user_details["user"]["username"], user.username);
    //     assert_eq!(user_details["user"]["email"], user.email);
    // }

    // #[async_test]
    // async fn update_and_retrieve_user_details() {
    //     let server = TestServer::new(Repo::new());
    //     let user = generate::new_user();

    //     let stored_user = register_user(&server, &user).await;
    //     let token = login_user(&server, &user).await;

    //     assert_eq!(stored_user["user"]["bio"], Value::Null);
    //     assert_eq!(stored_user["user"]["image"], Value::Null);

    //     let new_details = json!({
    //         "user": {
    //             "bio": "I like to code.",
    //             "image": "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
    //         }
    //     });
    //     let updated_user =  update_user_details(&server, &new_details, &token).await;
    //     assert_eq!(updated_user["user"]["bio"], new_details["user"]["bio"]);
    //     assert_eq!(updated_user["user"]["image"], new_details["user"]["image"]);

    //     let user_details =  get_user_details(&server, &token).await;
    //     assert_eq!(user_details["user"]["bio"], new_details["user"]["bio"]);
    //     assert_eq!(user_details["user"]["image"], new_details["user"]["image"]);
    // }

    async fn register_user<'a>(server: &'a TestServer, user: &'a NewUser) -> Value {
        let res = server
            .call(
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
                        .into(),
                    )
                    .unwrap(),
            )
            .await;
        response_json(res).await
    }

    async fn login_user<'a>(server: &'a TestServer, user: &'a NewUser) -> String {
        let res = server
            .call(
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
                    .unwrap(),
            )
            .await;
        assert_eq!(res.status(), 200);

        let response_json = response_json(res).await;

        assert!(response_json["user"]["token"].is_string());
        response_json["user"]["token"]
            .as_str()
            .expect("Token not found")
            .to_string()
    }

    async fn get_user_details<'a>(server: &'a TestServer, token: &'a String) -> Value {
        let auth_header = format!("token: {}", token);
        let res = server
            .call(
                Request::get("/api/user")
                    .header("Authorization", auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(res.status(), 200);
        response_json(res).await
    }

    async fn update_user_details<'a>(
        server: &'a TestServer,
        details: &'a Value,
        token: &'a String,
    ) -> Value {
        let res = server
            .call(
                Request::put("/api/user")
                    .header("Authorization", format!("token: {}", token))
                    .body(details.to_string().into())
                    .unwrap(),
            )
            .await;
        assert_eq!(res.status(), 200);
        response_json(res).await
    }
}
