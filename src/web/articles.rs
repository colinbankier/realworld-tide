use http::status::StatusCode;
use tide::{self, body::Json, AppData};

use crate::conduit::{articles, articles::ArticleQuery};
use crate::db::Repo;
use crate::models::*;
use crate::query_string::UrlQuery;
use crate::web::diesel_error;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ArticleResponse {
    articles: Vec<Article>,
}

pub async fn list_articles(
    repo: AppData<Repo>,
    query: UrlQuery<ArticleQuery>,
) -> Result<Json<ArticleResponse>, StatusCode> {
    let result = articles::find(repo.0, query.0).await;

    result
        .map(|articles| Json(ArticleResponse { articles }))
        .map_err(|e| diesel_error(&e))
}

#[cfg(test)]
mod tests {
    // These tests are "integration" tests that exercise a workflow via the http service.
    use crate::db::Repo;
    use crate::test_helpers::test_server::response_json;
    use crate::test_helpers::test_server::TestServer;
    use crate::test_helpers::{create_articles, create_users};
    use http::Request;
    use http_service::Body;
    use serde_json::Value;
    use tokio_async_await_test::async_test;

    #[async_test]
    async fn should_list_articles() {
        let repo = Repo::new();
        let server = TestServer::new(repo.clone());

        let users =  create_users(&repo, 5).await ;
        let _articles =  create_articles(&repo, users).await;

        let articles_list =  get_articles(&server, None).await ;

        match &articles_list["articles"] {
            Value::Array(ref list) => assert_eq!(list.len(), 5),
            _ => panic!(format!("Unexpected article response. {}", &articles_list)),
        }
    }

    #[async_test]
    async fn should_get_articles_by_author() {
        let repo = Repo::new();
        let server = TestServer::new(repo.clone());

        let users =  create_users(&repo, 5).await ;
        let articles =  create_articles(&repo, users.clone()).await;

        let articles_list =
             get_articles(&server, Some(format!("author={}", users[0].username))).await ;

        match &articles_list["articles"] {
            Value::Array(ref list) => {
                assert_eq!(list[0]["title"], articles[0].title);
                assert_eq!(list.len(), 1);
            }
            _ => panic!(format!("Unexpected article response. {}", &articles_list)),
        }
    }

    async fn get_articles<'a>(server: &'a TestServer, query: Option<String>) -> Value {
        let url = match query {
            None => "/api/articles".to_string(),
            Some(qs) => format!("/api/articles?{}", qs),
        };
        let res = server.call(Request::get(url).body(Body::empty()).unwrap()).await;
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
