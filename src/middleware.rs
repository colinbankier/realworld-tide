use futures::future::FutureObj;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::time::{Duration, Instant};
use http::status::StatusCode;
use jsonwebtoken::{decode, Validation, TokenData};

use tide::{middleware::RequestContext, Middleware, Response, IntoResponse};

pub struct JWTMiddleware<T> {
    secret: &'static str,
    validation: Validation,
    claims: PhantomData<T>,
}

impl<T> JWTMiddleware<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    /// Creates a JWTMiddleware instance from the provided secret,
    /// which, by default, uses HS256 as the crypto scheme.
    pub fn new(secret: &'static str) -> Self {
        let validation = Validation::default();

        JWTMiddleware {
            secret,
            validation,
            claims: PhantomData,
        }
    }

    /// Create a new instance of the middleware by appending new
    /// validation constraints.
    pub fn validation(self, validation: Validation) -> Self {
        JWTMiddleware { validation, ..self }
    }
}

impl<Data: Clone + Send> Middleware<Data> for JWTMiddleware {
    fn handle<'a>(&'a self, ctx: RequestContext<'a, Data>) -> FutureObj<'a, Response> {
        FutureObj::new(Box::new(
            async move {
            let token = match ctx.req.headers().get("Authorization") {
                Some(h) => match h.to_str() {
                    Ok(hx) => hx.get(8..),
                    _ => None,
                },
                _ => None,
            };
                info!("JWT token: {}", token);
        if token.is_none() {
            return StatusCode::BAD_REQUEST.into_response();
        }

        match decode::<T>(&token.unwrap(), self.secret.as_ref(), &self.validation) {
            Ok(token) => {
                await!(ctx.next())
            }
            Err(e) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
        }}
        ))
    }
}
