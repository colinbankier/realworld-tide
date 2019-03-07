use http::status::StatusCode;
use tide::{self, body::Json, AppData};

use crate::conduit::{articles, articles::ArticleQuery};
use crate::db::Repo;
use crate::models::*;
use crate::query_string::UrlQuery;
use crate::web::diesel_error;

#[derive(Serialize)]
pub struct ArticleResponse {
    articles: Vec<Article>,
}

pub async fn list_articles(
    repo: AppData<Repo>,
    query: UrlQuery<ArticleQuery>,
) -> Result<Json<ArticleResponse>, StatusCode> {
    let result = await! { articles::find(repo.0, query.0) };

    result
        .map(|articles| Json(ArticleResponse{articles))
        .map_err(|e| diesel_error(&e))
}

#[cfg(test)]
mod tests {
    // These tests are "integration" tests that exercise a workflow via the http service.
    use crate::db::Repo;
    use crate::test_helpers::test_server::TestServer;
    use crate::test_helpers::{create_articles, create_users, init_env};
    use http::Request;
    use http_service::Body;
    use serde_json::{json, Value};
    use std::str::from_utf8;
    use tokio_async_await_test::async_test;
    use crate::test_helpers::test_server::response_json;

    #[async_test]
    async fn should_list_articles() {
        init_env();
        let repo = Repo::new();
        let server = TestServer::new(repo.clone());

        let users = await! { create_users(&repo, 5) };
        let articles = await! { create_articles(&repo, users)};

        let articles_list = await! { get_articles(&server) };

        match &articles_list["articles"] {
            Value::Array(ref list) => assert_eq!(list.len(), 5, "json articles"),
            _ => panic!(format!("Unexpected article response. {}", &articles_list)),
        }
    }

    // #[async_test]
    // async fn should_get_articles_by_author() {
    //     init_env();
    //     let repo = Repo::new();

    //     let users = await! { create_users(&repo, 5) };
    //     let _articles = await! { create_articles(&repo, users)};

    //     let articles_list =
    //         await! { list_articles(AppData(repo.clone()), UrlQuery(ArticleQuery::default()))}
    //             .expect("Failed to list articles");

    //     assert_eq!(articles_list.len(), 5);
    // }

    async fn get_articles<'a>(server: &'a TestServer) -> Value {
        let res = await!(server.call(Request::get("/api/articles")
            .body(Body::empty())
            .unwrap()));
        assert_eq!(res.status(), 200);
        await!(response_json(res))
    }

    // async fn login_user(repo: Repo, user: NewUser) -> Claims {
    //     let login_request = Json(AuthRequest {
    //         user: AuthUser {
    //             email: user.email,
    //             password: user.password,
    //         },
    //     });

    //     let login_response = await! { login(AppData(repo), login_request)};
    //     let stored_user = login_response.expect("User login failed").0.user;
    //     assert!(stored_user.token.is_some());

    //     auth::claims_for(stored_user.id, 60)
    // }

    // async fn update_user_details(repo: Repo, new_details: UpdateUser, auth: Claims) -> User {
    //     let user_json = Json(UpdateUserRequest { user: new_details });
    //     let update_response = await! {
    //         update_user(AppData(repo), user_json, auth)
    //     }
    //     .expect("Update user failed");
    //     update_response.0.user
    // }
}
