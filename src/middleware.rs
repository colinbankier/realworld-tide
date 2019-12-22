use futures::future::BoxFuture;
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
    fn get_claims(&mut self) -> Result<Claims, Error>;
}

impl<State> ContextExt for Request<State> {
    fn get_claims(&mut self) -> Result<Claims, Error> {
        let claims = self.local::<Claims>().expect("Missing auth middleware");
        Ok(claims.clone())
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for JwtMiddleware {
    fn handle<'a>(
        &'a self,
        cx: Request<State>,
        next: Next<'a, State>,
    ) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            let claims = extract_claims(cx.headers());
            if let Some(c) = claims {
                return next.run(cx.set_local(c)).await;
            } else {
                return Response::new(403);
            }
        })
    }
}
