use crate::conduit::{articles, articles::ArticleQuery};
use crate::models::*;
use crate::web::diesel_error;
use crate::Repo;
use serde_derive::Serialize;
use tide::{querystring::ContextExt, response, Context, EndpointResult};

#[derive(Serialize)]
pub struct ArticleResponse {
    articles: Vec<Article>,
}

pub async fn list_articles(cx: Context<Repo>) -> EndpointResult {
    let query = cx.url_query::<ArticleQuery>()?;
    let repo = cx.state();
    let result = articles::find(repo, query).await;

    result
        .map(|articles| response::json(articles))
        .map_err(|e| diesel_error(&e))
}

#[cfg(test)]
mod tests {
    // These tests are "integration" tests that exercise a workflow via the http service.
    use crate::db::Repo;
    use crate::test_helpers::test_server::{get_repo, response_json, TestServer};
    use crate::test_helpers::{create_articles, create_users};
    use futures::executor::block_on;
    use futures::executor::ThreadPool;
    use http::Request;
    use http_service::Body;
    use serde_json::Value;
    use std::sync::Arc;

    #[test]
    fn should_list_articles() {
        let runtime = ThreadPool::new().unwrap();
        runtime.spawn_ok(async move {
            let server = TestServer::new(get_repo());
            let repo = get_repo();
            let users = create_users(&repo, 5).await;
            let _articles = create_articles(&repo, users).await;
            let articles_list = get_articles(&server, None).await;

            match &articles_list["articles"] {
                Value::Array(ref list) => assert_eq!(list.len(), 5),
                _ => panic!(format!("Unexpected article response. {}", &articles_list)),
            }
        })
    }

    #[test]
    fn should_get_articles_by_author() {
        let runtime = ThreadPool::new().unwrap();
        runtime.spawn_ok(async move {
            let server = TestServer::new(get_repo());
            let repo = get_repo();
            let users = create_users(&repo, 5).await;
            let articles = create_articles(&repo, users.clone()).await;

            let query = Some(format!("author={}", users[0].username));
            let articles_list = get_articles(&server, query).await;

            match &articles_list["articles"] {
                Value::Array(ref list) => {
                    assert_eq!(list[0]["title"], articles[0].title);
                    assert_eq!(list.len(), 1);
                }
                _ => panic!(format!("Unexpected article response. {}", &articles_list)),
            }
        })
    }

    async fn get_articles<'a>(server: &'a TestServer, query: Option<String>) -> Value {
        let url = match query {
            None => "/api/articles".to_string(),
            Some(qs) => format!("/api/articles?{}", qs),
        };
        let res = server
            .call(Request::get(url).body(Body::empty()).unwrap())
            .await;
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
}
