use crate::db::Repo;
use crate::set_routes;
use futures::prelude::*;
use http_service::{HttpService, Request, Response};
use serde_json::Value;
use std::str::from_utf8;
use tide::Server;

pub type TestServer = TestBackend<Server<Repo>>;

pub struct TestBackend<T: HttpService> {
    service: T,
}

impl<T: HttpService> TestBackend<T> {
    fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
        Ok(Self { service })
    }

    pub async fn call(&self, req: Request) -> Response {
        let mut connection = await! {self.service.connect().into_future()}.ok().unwrap();
        let response = await! { self.service.respond(&mut connection, req).into_future() };
        response.ok().unwrap()
    }
}

// TODO: separate app specific logic
impl TestServer {
    pub fn new(repo: Repo) -> TestServer {
        let app = crate::set_routes(tide::App::new(repo));
        let app = set_routes(app);
        TestBackend::wrap(app.into_http_service()).unwrap()
    }
}

pub async fn response_json(res: Response) -> Value {
    let body = await! { res.into_body().into_vec() }.unwrap();
    serde_json::from_str(from_utf8(&body).unwrap()).expect("Could not parse body.")
}
