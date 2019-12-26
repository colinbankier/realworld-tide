// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use helpers::generate;
use helpers::test_server::{response_json, TestApp, TestServer};
use helpers::{create_articles, create_users};

use async_std::task;
use http::{Request, Response};
use http_service::Body;
use realworld_tide::conduit::users;
use realworld_tide::db::models::NewArticle;
use serde_json::{json, Value};

#[test]
fn should_list_articles() {
    let mut server = TestApp::new();
    task::block_on(async move {
        let users = create_users(&server.repository, 5);
        let _articles = create_articles(&server.repository, users);
        let articles_list = get_articles(&mut server.server, None).await;

        match &articles_list["articles"] {
            Value::Array(ref list) => assert_eq!(list.len(), 5),
            _ => panic!(format!("Unexpected article response. {}", &articles_list)),
        }
    })
}

#[test]
fn should_get_articles_by_author() {
    let mut server = TestApp::new();
    task::block_on(async move {
        let users = create_users(&server.repository, 5);
        let articles = create_articles(&server.repository, users.clone());

        let query = Some(format!("author={}", users[0].username));
        let articles_list = get_articles(&mut server.server, query).await;

        match &articles_list["articles"] {
            Value::Array(ref list) => {
                assert_eq!(list.len(), 1);
                assert_eq!(list[0]["title"], articles[0].title);
            }
            _ => panic!(format!("Unexpected article response. {}", &articles_list)),
        }
    })
}

#[test]
fn should_create_article() {
    let mut server = TestApp::new();
    task::block_on(async move {
        let user = generate::new_user();
        let user = users::insert(&server.repository, user).expect("Failed to create user");

        let article = generate::new_article(user.id);
        let response = create_article(&mut server.server, &article).await;
        assert!(response.status().is_success());
        assert!(false);

        let query = Some(format!("author={}", user.username));
        let articles_list = get_articles(&mut server.server, query).await;

        match &articles_list["articles"] {
            Value::Array(ref list) => {
                assert_eq!(list.len(), 1);
                assert_eq!(list[0]["title"], article.title);
                assert_eq!(list[0]["description"], article.description);
                assert_eq!(list[0]["body"], article.body);
            }
            _ => panic!(format!("Unexpected article response. {}", &articles_list)),
        }
    })
}

async fn create_article(server: &mut TestServer, article: &NewArticle) -> Response<Body> {
    let body = json!({
        "article": {
            "title": article.title,
            "description": article.description,
            "body": article.body,
        }
    });
    let res = server
        .simulate(
            Request::post("/api/articles")
                .body(body.to_string().into_bytes().into())
                .unwrap(),
        )
        .unwrap();
    res
}

async fn get_articles(server: &mut TestServer, query: Option<String>) -> Value {
    let url = match query {
        // Adding a useless query parameter to avoid an issue in Tide's parsing
        // See https://github.com/http-rs/tide/pull/384
        None => format!("/api/articles?random=random"),
        Some(qs) => format!("/api/articles?{}", qs),
    };
    let res = server
        .simulate(Request::get(url).body(Body::empty()).unwrap())
        .unwrap();
    assert_eq!(res.status(), 200);
    response_json(res).await
}

// async fn login_user(repo: Repo, user: NewUser) -> Claims {
//     let login_request = Json(AuthRequest {
//         user: AuthUser {
//             email: user.email,
//             password: user.password,
//         },
//     });

//     let login_response =  login(AppData(repo), login_request).await;
//     let stored_user = login_response.expect("User login failed").0.user;
//     assert!(stored_user.token.is_some());

//     auth::claims_for(stored_user.id, 60)
// }

// async fn update_user_details(repo: Repo, new_details: UpdateUser, auth: Claims) -> User {
//     let user_json = Json(UpdateUserRequest { user: new_details });
//     let update_response =
//         update_user(AppData(repo), user_json, auth).await
//     .expect("Update user failed");
//     update_response.0.user
// }
