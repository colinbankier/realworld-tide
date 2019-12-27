// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use helpers::generate;
use helpers::test_server::{response_json, TestApp, TestServer};

use async_std::task;
use http::Request;
use http_service::Body;
use serde_json::{json, Value};

#[test]
fn register_and_login() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let user = generate::new_user();

        server.register_user(&user).await.unwrap();
        let token = server.login_user(&user).await.unwrap().user.token;
        let user_details = get_user_details(&mut server.server, &token).await;

        assert_eq!(user_details["user"]["username"], user.username);
        assert_eq!(user_details["user"]["email"], user.email);
    })
}

#[test]
fn update_and_retrieve_user_details() {
    task::block_on(async move {
        let mut server = TestApp::new();
        let user = generate::new_user();

        let stored_user = server.register_user(&user).await.unwrap();
        let token = server.login_user(&user).await.unwrap().user.token;

        assert_eq!(stored_user.user.bio, None);
        assert_eq!(stored_user.user.image, None);

        let new_details = json!({
            "user": {
                "bio": "I like to code.",
                "image": "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
            }
        });
        let updated_user = update_user_details(&mut server.server, &new_details, &token).await;
        assert_eq!(updated_user["user"]["bio"], new_details["user"]["bio"]);
        assert_eq!(updated_user["user"]["image"], new_details["user"]["image"]);

        let user_details = get_user_details(&mut server.server, &token).await;
        assert_eq!(user_details["user"]["bio"], new_details["user"]["bio"]);
        assert_eq!(user_details["user"]["image"], new_details["user"]["image"]);
    })
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
