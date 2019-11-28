use crate::db::Repo;
use crate::set_routes;

use crate::configuration::Settings;
use diesel::PgConnection;
use http_service::Response;
use http_service_mock::{make_server, TestBackend};
use serde_json::Value;
use std::str::from_utf8;
use tide::Server;

pub type TestServer = TestBackend<Server<Repo<PgConnection>>>;

// TODO: separate app specific logic
pub fn new(repo: Repo<PgConnection>) -> TestServer {
    let app = crate::set_routes(tide::App::with_state(repo));
    let app = set_routes(app);
    make_server(app.into_http_service()).unwrap()
}

pub fn get_repo() -> Repo<PgConnection> {
    let settings = Settings::new().expect("Failed to load configuration");
    return Repo::new(&settings.database.connection_string());
}

pub async fn response_json(res: Response) -> Value {
    let body = res.into_body().into_vec().await.unwrap();
    serde_json::from_str(from_utf8(&body).unwrap()).expect("Could not parse body.")
}
