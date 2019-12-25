use futures::future::BoxFuture;
use http::status::StatusCode;
use log::info;
use tide::{Error, Middleware, Next, Request, Response};

use crate::auth::{extract_claims, Claims};

#[derive(Clone, Default, Debug)]
pub struct JwtMiddleware {}

#[allow(dead_code)]
impl JwtMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ContextExt {
    fn get_claims(&self) -> Result<&Claims, Error>;
}

impl<State> ContextExt for Request<State> {
    fn get_claims(&self) -> Result<&Claims, Error> {
        self.local::<Claims>()
            .ok_or(Error::from(StatusCode::UNAUTHORIZED))
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for JwtMiddleware {
    fn handle<'a>(&'a self, cx: Request<State>, next: Next<'a, State>) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            info!("Headers: {:?}", cx.headers());
            let claims = extract_claims(cx.headers());
            info!("Claims: {:?}", claims);
            if let Some(c) = claims {
                return next.run(cx.set_local(c)).await;
            } else {
                return next.run(cx).await;
            }
        })
    }
}
