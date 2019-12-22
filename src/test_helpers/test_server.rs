use crate::db::Repo;
use crate::set_routes;

use crate::configuration::Settings;
use diesel::PgConnection;
use http_service::Response;
use http_service_mock::{make_server, TestBackend};
use serde_json::Value;
use std::io::Read;
use async_std::io::prelude::ReadExt;
use tide::server::Service;

pub type TestServer = TestBackend<Service<Repo<PgConnection>>>;

pub fn new(repo: Repo<PgConnection>) -> TestServer {
    let app = crate::set_routes(tide::with_state(repo));
    let app = set_routes(app);
    make_server(app.into_http_service()).unwrap()
}

pub fn get_repo() -> Repo<PgConnection> {
    let settings = Settings::new().expect("Failed to load configuration");
    return Repo::new(&settings.database.connection_string());
}

pub async fn response_json(mut res: Response) -> Value {
    let mut body = String::new();
    res.body().read_to_string(&mut body);
    serde_json::from_str(&body).expect("Could not parse body.")
}
