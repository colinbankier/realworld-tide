use futures::future::BoxFuture;
use tide::{
    Error, Request,
    Middleware, Next,
    Response,
};

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
        let claims = self
            .extensions()
            .get::<Claims>()
            .ok_or_else(|| Error("Auth middleware missing".to_owned()))?;
        Ok(claims.clone())
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for JwtMiddleware {
    fn handle<'a>(
        &'a self,
        mut cx: Request<State>,
        next: Next<'a, State>,
    ) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            let claims = extract_claims(cx.headers());
            if let Some(c) = claims {
                // The `let _ = ...` is a workaround for issue: https://github.com/rustasync/tide/issues/278
                // Solution is according to suggestion in https://github.com/rust-lang/rust/issues/61579#issuecomment-500436524
                let _ = cx.extensions_mut().insert(c);
                return next.run(cx).await;
            } else {
                return Response::new(403);
            }
        })
    }
}
