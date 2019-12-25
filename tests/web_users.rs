// These tests are "integration" tests that exercise a workflow via the http service.

use realworld_tide::models::NewUser;

mod helpers;

use helpers::generate;
use helpers::test_db::get_repo;
use helpers::test_server::{new, response_json, TestServer};

use futures_executor::ThreadPool;
use http::Request;
use http_service::Body;
use serde_json::{json, Value};

#[test]
fn register_and_login() {
    let runtime = ThreadPool::new().unwrap();
    runtime.spawn_ok(async move {
        let mut server = new(get_repo());
        let user = generate::new_user();

        register_user(&mut server, &user).await;
        let token = login_user(&mut server, &user).await;
        let user_details = get_user_details(&mut server, &token).await;

        assert_eq!(user_details["user"]["username"], user.username);
        assert_eq!(user_details["user"]["email"], user.email);
    })
}

#[test]
fn update_and_retrieve_user_details() {
    let runtime = ThreadPool::new().unwrap();
    runtime.spawn_ok(async move {
        let mut server = new(get_repo());
        let user = generate::new_user();

        let stored_user = register_user(&mut server, &user).await;
        let token = login_user(&mut server, &user).await;

        assert_eq!(stored_user["user"]["bio"], Value::Null);
        assert_eq!(stored_user["user"]["image"], Value::Null);

        let new_details = json!({
            "user": {
                "bio": "I like to code.",
                "image": "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
            }
        });
        let updated_user = update_user_details(&mut server, &new_details, &token).await;
        assert_eq!(updated_user["user"]["bio"], new_details["user"]["bio"]);
        assert_eq!(updated_user["user"]["image"], new_details["user"]["image"]);

        let user_details = get_user_details(&mut server, &token).await;
        assert_eq!(user_details["user"]["bio"], new_details["user"]["bio"]);
        assert_eq!(user_details["user"]["image"], new_details["user"]["image"]);
    })
}

async fn register_user(server: &mut TestServer, user: &NewUser) -> Value {
    let res = server
        .simulate(
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
                    .into_bytes()
                    .into(),
                )
                .unwrap(),
        )
        .unwrap();
    response_json(res).await
}

async fn login_user(server: &mut TestServer, user: &NewUser) -> String {
    let res = server
        .simulate(
            Request::post("/api/users/login")
                .body(
                    json!({
                        "user": {
                            "email": user.email,
                            "password": user.password,
                        }
                    })
                    .to_string()
                    .into_bytes()
                    .into(),
                )
                .unwrap(),
        )
        .unwrap();
    assert_eq!(res.status(), 200);

    let response_json = response_json(res).await;

    assert!(response_json["user"]["token"].is_string());
    response_json["user"]["token"]
        .as_str()
        .expect("Token not found")
        .to_string()
}

async fn get_user_details(server: &mut TestServer, token: &String) -> Value {
    let auth_header = format!("token: {}", token);
    let res = server
        .simulate(
            Request::get("/api/user")
                .header("Authorization", auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .unwrap();
    assert_eq!(res.status(), 200);
    response_json(res).await
}

async fn update_user_details(server: &mut TestServer, details: &Value, token: &String) -> Value {
    let res = server
        .simulate(
            Request::put("/api/user")
                .header("Authorization", format!("token: {}", token))
                .body(details.to_string().into_bytes().into())
                .unwrap(),
        )
        .unwrap();
    assert_eq!(res.status(), 200);
    response_json(res).await
}
