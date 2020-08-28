use crate::Context;
use domain::repositories::Repository;
use http::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::{Response, Server};

// pub fn result_to_response<T: IntoResponse, E: IntoResponse>(r: Result<T, E>) -> Response {
//     match r {
//         Ok(r) => r.into_response(),
//         Err(r) => {
//             let res = r.into_response();
//             if res.status().is_success() {
//                 panic!(
//                     "Attempted to yield error response with success code {:?}",
//                     res.status()
//                 )
//             }
//             res
//         }
//     }
// }

pub fn get_app<R: Repository + Send + Sync>(repository: R) -> Server<Context<R>> {
    let context = Context { repository };
    let mut app = Server::with_state(context);
    app = add_middleware(app);
    app = add_routes(app);
    app
}

pub fn add_routes<R: Repository + Send + Sync>(mut api: Server<Context<R>>) -> Server<Context<R>> {
    api.at("/api/user")
        .get(|req| async move { crate::users::get_current_user(req).await })
        .put(|req| async move { crate::users::update_user(req).await });
    api.at("/api/users")
        .post(|req| async move { crate::users::register(req).await });
    api.at("/api/users/login")
        .post(|req| async move { crate::users::login(req).await });
    api.at("/api/profiles/:username")
        .get(|req| async move { crate::profiles::get_profile(req).await });
    api.at("/api/profiles/:username/follow")
        .post(|req| async move { crate::profiles::follow(req).await })
        .delete(|req| async move { crate::profiles::unfollow(req).await });
    api.at("/api/tags")
        .get(|req| async move { crate::articles::tags(req).await });
    api.at("/api/articles")
        .get(|req| async move { crate::articles::list_articles(req).await })
        .post(|req| async move { crate::articles::insert_article(req).await });
    api.at("/api/articles/feed")
        .get(|req| async move { crate::articles::feed(req).await });
    api.at("/api/articles/:slug")
        .get(|req| async move { crate::articles::get_article(req).await })
        .put(|req| async move { crate::articles::update_article(req).await })
        .delete(|req| async move { crate::articles::delete_article(req).await });
    api.at("/api/articles/:slug/comments")
        .get(|req| async move { crate::comments::get(req).await })
        .post(|req| async move { crate::comments::create(req).await });
    api.at("/api/articles/:slug/comments/:id")
        .delete(|req| async move { crate::comments::delete(req).await });
    api.at("/api/articles/:slug/favorite")
        .post(|req| async move { crate::articles::favorite(req).await })
        .delete(|req| async move { crate::articles::unfavorite(req).await });
    api
}

pub fn add_middleware<State: 'static + Sync + Send>(mut app: Server<State>) -> Server<State> {
    let rules = CorsMiddleware::new()
        .allow_methods(HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"))
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.middleware(tide::log::LogMiddleware::new());
    app.middleware(rules);
    app.middleware(crate::middleware::JwtMiddleware::new());
    app
}
