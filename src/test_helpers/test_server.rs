use crate::db::Repo;
use crate::set_routes;
use futures::prelude::*;
use http_service::{HttpService, Request, Response};
use tide::Server;
pub type TestServer = TestBackend<Server<Repo>>;

pub struct TestBackend<T: HttpService> {
    service: T,
}

impl<T: HttpService> TestBackend<T> {
    fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
        Ok(Self { service })
    }

    pub async fn simulate(&self, req: Request) -> Response {
        let mut connection = await! {self.service.connect().into_future()}.ok().unwrap();
        let response = await! { self.service.respond(&mut connection, req).into_future() };
        response.ok().unwrap()
    }
}

pub fn new(repo: Repo) -> TestServer {
    let app = crate::set_routes(tide::App::new(repo));
    let app = set_routes(app);
    TestBackend::wrap(app.into_http_service()).unwrap()
}
