pub mod users;

use http::status::StatusCode;
use tide::{self, body::Json, AppData};

use crate::conduit::{articles, articles::ArticleQuery};
use crate::db::Repo;
use crate::models::*;
use crate::query_string::UrlQuery;

pub async fn list_articles(
    repo: AppData<Repo>,
    query: UrlQuery<ArticleQuery>,
) -> Result<Json<Vec<Article>>, StatusCode> {
    let articles = await! { articles::find(repo.0, query.0) };

    articles
        .map(|article_list| Json(article_list))
        .map_err(|e| diesel_error(&e))
}

pub fn diesel_error(e: &diesel::result::Error) -> StatusCode {
    error!("{}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}

#[cfg(test)]
mod tests {
    // These tests are more like "integration" tests that exercise a workflow via the tide handlers.
    use super::*;
    use crate::auth;
    use crate::test_helpers::{create_articles, create_users, generate, init_env};
    use std::default::Default;
    use tokio_async_await_test::async_test;

    // #[async_test]
    // async fn should_list_articles() {
    //     let repo = Repo::new();

    //     let users = await! { create_users(&repo, 5) };
    //     let _articles = await! { create_articles(&repo, users)};

    //     let articles_list =
    //         await! { list_articles(AppData(repo.clone()), UrlQuery(ArticleQuery::default()))}
    //             .expect("Failed to list articles");

    //     assert_eq!(articles_list.len(), 5);
    // }

    // #[async_test]
    // async fn should_get_articles_by_author() {
    //     let repo = Repo::new();

    //     let users = await! { create_users(&repo, 5) };
    //     let _articles = await! { create_articles(&repo, users)};

    //     let articles_list =
    //         await! { list_articles(AppData(repo.clone()), UrlQuery(ArticleQuery::default()))}
    //             .expect("Failed to list articles");

    //     assert_eq!(articles_list.len(), 5);
    // }
    // async fn register_user(repo: Repo, user: NewUser) -> User {
    //     let reg_request = Json(Registration { user: user });
    //     let reg_response = await! { register(AppData(repo), reg_request) };
    //     reg_response.expect("Registration failed").0.user
    // }

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
