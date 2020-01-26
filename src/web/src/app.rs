use crate::Context;
use domain::repositories::Repository;
use tide::{IntoResponse, Response, Server};

pub fn result_to_response<T: IntoResponse, E: IntoResponse>(r: Result<T, E>) -> Response {
    match r {
        Ok(r) => r.into_response(),
        Err(r) => {
            let res = r.into_response();
            if res.status().is_success() {
                panic!(
                    "Attempted to yield error response with success code {:?}",
                    res.status()
                )
            }
            res
        }
    }
}

pub fn get_app<R: Repository + Send + Sync>(repository: R) -> Server<Context<R>> {
    let context = Context { repository };
    let mut app = Server::with_state(context);
    app = add_middleware(app);
    app = add_routes(app);
    app
}

pub fn add_routes<R: Repository + Send + Sync>(mut app: Server<Context<R>>) -> Server<Context<R>> {
    app.at("/api").nest(|api| {
        api.at("/user")
            .get(|req| async move { result_to_response(crate::users::get_current_user(req).await) })
            .put(|req| async move { result_to_response(crate::users::update_user(req).await) });
        api.at("/users")
            .post(|req| async move { result_to_response(crate::users::register(req).await) });
        api.at("/users/login")
            .post(|req| async move { result_to_response(crate::users::login(req).await) });
        api.at("/profiles/:username")
            .get(|req| async move { result_to_response(crate::profiles::get_profile(req).await) });
        api.at("/profiles/:username/follow")
            .post(|req| async move { result_to_response(crate::profiles::follow(req).await) })
            .delete(|req| async move { result_to_response(crate::profiles::unfollow(req).await) });
        api.at("/tags")
            .get(|req| async move { result_to_response(crate::articles::tags(req).await) });
        api.at("/articles")
            .get(|req| async move { result_to_response(crate::articles::list_articles(req).await) })
            .post(
                |req| async move { result_to_response(crate::articles::insert_article(req).await) },
            );
        api.at("/articles/feed")
            .get(|req| async move { result_to_response(crate::articles::feed(req).await) });
        api.at("/articles/:slug")
            .get(|req| async move { result_to_response(crate::articles::get_article(req).await) })
            .put(
                |req| async move { result_to_response(crate::articles::update_article(req).await) },
            )
            .delete(
                |req| async move { result_to_response(crate::articles::delete_article(req).await) },
            );
        api.at("/articles/:slug/comments")
            .get(|req| async move { result_to_response(crate::comments::get(req).await) })
            .post(|req| async move { result_to_response(crate::comments::create(req).await) });
        api.at("/articles/:slug/comments/:id")
            .delete(|req| async move { result_to_response(crate::comments::delete(req).await) });
        api.at("/articles/:slug/favorite")
            .post(|req| async move { result_to_response(crate::articles::favorite(req).await) })
            .delete(
                |req| async move { result_to_response(crate::articles::unfavorite(req).await) },
            );
    });
    app
}

pub fn add_middleware<State: 'static + Sync + Send>(mut app: Server<State>) -> Server<State> {
    app.middleware(tide::middleware::RequestLogger::new());
    app.middleware(crate::middleware::JwtMiddleware::new());
    app
}