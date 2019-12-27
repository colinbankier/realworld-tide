use realworld_tide::db::models::NewUser;
use realworld_tide::db::Repo;
use realworld_tide::web::get_app;
use realworld_tide::web::users::responses::UserResponse;

use crate::helpers::test_db::{clean_db, get_repo};
use async_std::io::prelude::ReadExt;
use diesel::PgConnection;
use http_service::Response;
use http_service_mock::{make_server, TestBackend};
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

    pub async fn register_user(&mut self, user: &NewUser) -> Result<UserResponse, Response> {
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
        if response.status().is_success() {
            let registered_user: UserResponse = response_json(response).await;
            Ok(registered_user)
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

pub async fn response_json<T: DeserializeOwned>(mut res: Response) -> T {
    let mut body = String::new();
    res.body_mut()
        .read_to_string(&mut body)
        .await
        .expect("Failed to read body.");
    serde_json::from_str(&body).expect("Could not parse body.")
}
