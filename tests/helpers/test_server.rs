use realworld_tide::db::Repo;
use realworld_tide::web::get_app;

use async_std::io::prelude::ReadExt;
use diesel::PgConnection;
use http_service::Response;
use http_service_mock::{make_server, TestBackend};
use serde_json::Value;
use tide::server::Service;

pub type TestServer = TestBackend<Service<Repo<PgConnection>>>;

pub fn new(repo: Repo<PgConnection>) -> TestServer {
    let app = get_app(repo);
    make_server(app.into_http_service()).unwrap()
}

pub async fn response_json(mut res: Response) -> Value {
    let mut body = String::new();
    res.body_mut().read_to_string(&mut body);
    serde_json::from_str(&body).expect("Could not parse body.")
}
