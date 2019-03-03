use crate::db::Repo;
use crate::set_routes;
use futures::{executor::block_on, prelude::*};
use http_service::{Body, HttpService, Request, Response};
use tide::{
    head::{Named, NamedSegment},
    Server,
};

pub struct TestBackend<T: HttpService> {
    service: T,
    connection: T::Connection,
}

impl<T: HttpService> TestBackend<T> {
    fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
        let connection = block_on(service.connect().into_future())?;
        Ok(Self {
            service,
            connection,
        })
    }

    pub fn simulate(&mut self, req: Request) -> Result<Response, <T::Fut as TryFuture>::Error> {
        block_on(
            self.service
                .respond(&mut self.connection, req)
                .into_future(),
        )
    }
}

pub fn new(repo: Repo) -> TestBackend<Server<Repo>> {
    let app = crate::set_routes(tide::App::new(repo));
    let app = set_routes(app);
    TestBackend::wrap(app.into_http_service()).unwrap()
}
