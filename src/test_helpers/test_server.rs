use crate::db::Repo;
use crate::set_routes;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use futures::prelude::*;
use http_service::{HttpService, Request, Response};
use serde_json::Value;
use std::env;
use std::str::from_utf8;
use tide::Server;

pub type TestServer = TestBackend<Server<Repo<PgConnection>>>;

pub struct TestBackend<T: HttpService> {
    service: T,
}

impl<T: HttpService> TestBackend<T> {
    fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
        Ok(Self { service })
    }

    pub async fn call(&self, req: Request) -> Response {
        let mut connection = self.service.connect().into_future().await.ok().unwrap();
        let response = self
            .service
            .respond(&mut connection, req)
            .into_future()
            .await;
        response.ok().unwrap()
    }
}

// TODO: separate app specific logic
impl TestServer {
    pub fn new(repo: Repo<PgConnection>) -> TestServer {
        let app = crate::set_routes(tide::App::with_state(repo));
        let app = set_routes(app);
        TestBackend::wrap(app.into_http_service()).unwrap()
    }
}

pub fn get_repo() -> Repo<PgConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return Repo::new(&database_url);
}

pub async fn response_json(res: Response) -> Value {
    let body = res.into_body().into_vec().await.unwrap();
    serde_json::from_str(from_utf8(&body).unwrap()).expect("Could not parse body.")
}
