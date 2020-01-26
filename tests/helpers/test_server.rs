use realworld_tide::db::Repo;
use realworld_tide::web::get_app;
use realworld_tide::web::users::responses::UserResponse;

use crate::helpers::test_db::{clean_db, get_repo};
use async_std::io::prelude::ReadExt;
use diesel::PgConnection;
use http_service::Response;
use http_service_mock::{make_server, TestBackend};
use realworld_tide::domain::articles::ArticleQuery;
use realworld_tide::domain::SignUp;
use realworld_tide::web::articles::responses::{ArticleResponse, ArticlesResponse};
use realworld_tide::web::comments::responses::{CommentResponse, CommentsResponse};
use realworld_tide::web::profiles::responses::ProfileResponse;
use serde::de::DeserializeOwned;
use serde_json::json;
use tide::server::Service;

pub type TestServer = TestBackend<Service<Repo<PgConnection>>>;

pub struct TestApp {
    pub server: TestServer,
    pub repository: Repo<PgConnection>,
}

impl TestApp {
    pub fn new() -> Self {
        let app = get_app(get_repo());
        let server = make_server(app.into_http_service()).unwrap();
        Self {
            server,
            repository: get_repo(),
        }
    }

    pub async fn register_user(&mut self, user: &SignUp) -> Result<UserResponse, Response> {
        let response = self
            .server
            .simulate(
                http::Request::post("/api/users")
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
        response_json_if_success(response).await
    }

    pub async fn login_user(
        &mut self,
        email: &str,
        password: &str,
    ) -> Result<UserResponse, Response> {
        let response = self
            .server
            .simulate(
                http::Request::post("/api/users/login")
                    .body(
                        json!({
                            "user": {
                                "email": email.to_owned(),
                                "password": password.to_owned(),
                            }
                        })
                        .to_string()
                        .into_bytes()
                        .into(),
                    )
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_current_user(&mut self, token: &String) -> Result<UserResponse, Response> {
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::get("/api/user")
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn update_user_details(
        &mut self,
        details: &realworld_tide::web::users::update::Request,
        token: &String,
    ) -> Result<UserResponse, Response> {
        let response = self
            .server
            .simulate(
                http::Request::put("/api/user")
                    .header("Authorization", format!("token: {}", token))
                    .body(serde_json::to_string(details).unwrap().into_bytes().into())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn create_article(
        &mut self,
        article: &realworld_tide::web::articles::insert::Request,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let body = serde_json::to_string(article).unwrap();
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::post("/api/articles")
                    .header("Authorization", auth_header)
                    .body(body.into_bytes().into())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn update_article(
        &mut self,
        article: &realworld_tide::web::articles::update::Request,
        slug: &str,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}", slug);
        let body = serde_json::to_string(article).unwrap();
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::put(url)
                    .header("Authorization", auth_header)
                    .body(body.into_bytes().into())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_articles(
        &mut self,
        query: Option<ArticleQuery>,
    ) -> Result<ArticlesResponse, Response> {
        let query_string = serde_qs::to_string(&query).unwrap();
        let url = format!("/api/articles?{}", query_string);
        let response = self
            .server
            .simulate(
                http::Request::get(url)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_article(
        &mut self,
        slug: &str,
        token: Option<&str>,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}", slug);
        let request = match token {
            Some(token) => {
                let auth_header = format!("token: {}", token);
                http::Request::get(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap()
            }
            None => http::Request::get(url)
                .body(http_service::Body::empty())
                .unwrap(),
        };
        let response = self.server.simulate(request).unwrap();
        response_json_if_success(response).await
    }

    pub async fn delete_article(&mut self, slug: &str, token: &str) -> Result<(), Response> {
        let url = format!("/api/articles/{}", slug);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::delete(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response)
        }
    }

    pub async fn favorite_article(
        &mut self,
        slug: &str,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}/favorite", slug);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::post(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn unfavorite_article(
        &mut self,
        slug: &str,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}/favorite", slug);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::delete(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_profile(
        &mut self,
        username: &str,
        token: Option<&str>,
    ) -> Result<ProfileResponse, Response> {
        let url = format!("/api/profiles/{}", username);
        let request = match token {
            Some(token) => {
                let auth_header = format!("token: {}", token);
                http::Request::get(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap()
            }
            None => http::Request::get(url)
                .body(http_service::Body::empty())
                .unwrap(),
        };
        let response = self.server.simulate(request).unwrap();
        response_json_if_success(response).await
    }

    pub async fn follow_profile(
        &mut self,
        username: &str,
        token: &str,
    ) -> Result<ProfileResponse, Response> {
        let url = format!("/api/profiles/{}/follow", username);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::post(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn unfollow_profile(
        &mut self,
        username: &str,
        token: &str,
    ) -> Result<ProfileResponse, Response> {
        let url = format!("/api/profiles/{}/follow", username);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::delete(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_comments(
        &mut self,
        slug: &str,
        token: Option<&str>,
    ) -> Result<CommentsResponse, Response> {
        let url = format!("/api/articles/{}/comments", slug);
        let request = match token {
            Some(token) => {
                let auth_header = format!("token: {}", token);
                http::Request::get(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap()
            }
            None => http::Request::get(url)
                .body(http_service::Body::empty())
                .unwrap(),
        };
        let response = self.server.simulate(request).unwrap();
        response_json_if_success(response).await
    }

    pub async fn create_comment(
        &mut self,
        slug: &str,
        comment: &realworld_tide::web::comments::create::Request,
        token: &str,
    ) -> Result<CommentResponse, Response> {
        let url = format!("/api/articles/{}/comments", slug);
        let auth_header = format!("token: {}", token);
        let body = serde_json::to_string(comment).unwrap();
        let response = self
            .server
            .simulate(
                http::Request::post(url)
                    .header("Authorization", auth_header)
                    .body(body.into_bytes().into())
                    .unwrap(),
            )
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn delete_comment(
        &mut self,
        slug: &str,
        comment_id: &u64,
        token: &str,
    ) -> Result<(), Response> {
        let url = format!("/api/articles/{}/comments/{}", slug, comment_id);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .simulate(
                http::Request::delete(url)
                    .header("Authorization", auth_header)
                    .body(http_service::Body::empty())
                    .unwrap(),
            )
            .unwrap();
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response)
        }
    }
}

impl std::ops::Drop for TestApp {
    fn drop(&mut self) {
        println!("Cleaning");
        clean_db(&self.repository)
    }
}

pub async fn response_json_if_success<T: DeserializeOwned>(
    response: Response,
) -> Result<T, Response> {
    if response.status().is_success() {
        Ok(response_json(response).await)
    } else {
        Err(response)
    }
}

pub async fn response_json<T: DeserializeOwned>(mut res: Response) -> T {
    let mut body = String::new();
    res.body_mut()
        .read_to_string(&mut body)
        .await
        .expect("Failed to read body.");
    serde_json::from_str(&body).expect("Could not parse body.")
}
