use futures::future;
use tide::{configuration::Store, Extract, IntoResponse, Request, Response, RouteMatch};

/// An extractor for query string in URL
///
pub struct UrlQuery<T>(pub T);

impl<S, T> Extract<S> for UrlQuery<T>
where
    T: Send + std::str::FromStr + std::default::Default + 'static,
    S: 'static,
{
    type Fut = future::Ready<Result<Self, Response>>;
    fn extract(
        _data: &mut S,
        req: &mut Request,
        _params: &Option<RouteMatch<'_>>,
        _store: &Store,
    ) -> Self::Fut {
        req.uri()
            .query()
            .map_or(Some(Default::default()), |q| q.parse().ok())
            .map_or(
                future::err(http::status::StatusCode::BAD_REQUEST.into_response()),
                |q| future::ok(UrlQuery(q)),
            )
    }
}
