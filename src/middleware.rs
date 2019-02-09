use futures::future::FutureObj;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::time::{Duration, Instant};
use http::status::StatusCode;
use jsonwebtoken::{decode, Validation, TokenData};
use std::{marker::PhantomData};
use serde::de::Deserialize;

use tide::{middleware::RequestContext, Middleware, Response, IntoResponse};

use crate::auth::extract_claims;

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

impl<T, Data: Clone + Send> Middleware<Data> for JWTMiddleware<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync {
    fn handle<'a>(&'a self, ctx: RequestContext<'a, Data>) -> FutureObj<'a, Response> {
        FutureObj::new(Box::new(
            async move {
                await!(ctx.next())
                // Not sure yet why is won't compile:
                // if let Some(_valid) = extract_claims(ctx.req.headers()) {
                //     await!(ctx.next())
                // } else {
                //     StatusCode::UNAUTHORIZED.into_response()
                // }
        }
        ))
    }
}
