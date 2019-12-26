use realworld_tide::db::Repo;
use realworld_tide::web::get_app;

use crate::helpers::test_db::{clean_db, get_repo};
use async_std::io::prelude::ReadExt;
use diesel::PgConnection;
use http_service::Response;
use http_service_mock::{make_server, TestBackend};
use serde_json::Value;
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
}

impl std::ops::Drop for TestApp {
    fn drop(&mut self) {
        println!("Cleaning");
        clean_db(&self.repository)
    }
}

pub async fn response_json(mut res: Response) -> Value {
    let mut body = String::new();
    res.body_mut()
        .read_to_string(&mut body)
        .await
        .expect("Failed to read body.");
    serde_json::from_str(&body).expect("Could not parse body.")
}
