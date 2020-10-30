use log::info;
use tide::{Error, Middleware, Next, Request, Result, StatusCode};

use crate::auth::{extract_claims, Claims};

#[derive(Clone, Default, Debug)]
pub struct JwtMiddleware {}

impl JwtMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ContextExt {
    fn get_claims(&self) -> Result<&Claims>;
}

impl<State> ContextExt for Request<State> {
    fn get_claims(&self) -> Result<&Claims> {
        self.ext::<Claims>()
            .ok_or_else(|| Error::from_str(StatusCode::Unauthorized, "no"))
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for JwtMiddleware {
    async fn handle(&self, mut cx: Request<State>, next: Next<'_, State>) -> Result {
        info!("Headers: {:?}", cx.iter());
        let authorization = cx.header("Authorization").map(|v| v.last());
        let claims = authorization.map(|a| extract_claims(a));
        info!("Claims: {:?}", claims);
        let response = if let Some(c) = claims {
            cx.set_ext(c);
        };
        Ok(next.run(cx).await)
    }
}
