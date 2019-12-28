use crate::web;
use crate::Repo;
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

pub fn get_app(repository: Repo) -> Server<Repo> {
    let mut app = Server::with_state(repository);
    app = add_middleware(app);
    app = add_routes(app);
    app
}

pub fn add_routes(mut app: Server<Repo>) -> Server<Repo> {
    app.at("/api").nest(|api| {
        api.at("/user")
            .get(|req| async move { result_to_response(web::users::get_current_user(req).await) })
            .put(|req| async move { result_to_response(web::users::update_user(req).await) });
        api.at("/users")
            .post(|req| async move { result_to_response(web::users::register(req).await) });
        api.at("/users/login")
            .post(|req| async move { result_to_response(web::users::login(req).await) });
        api.at("/profiles/:username")
            .get(|req| async move { result_to_response(web::profiles::get_profile(req).await) });
        api.at("/profiles/:username/follow")
            .post(|req| async move { result_to_response(web::profiles::follow(req).await) })
            .delete(|req| async move { result_to_response(web::profiles::unfollow(req).await) });
        api.at("/tags")
            .get(|req| async move { result_to_response(web::articles::tags(req).await) });
        api.at("/articles")
            .get(|req| async move { result_to_response(web::articles::list_articles(req).await) })
            .post(
                |req| async move { result_to_response(web::articles::insert_article(req).await) },
            );
        api.at("/articles/feed")
            .get(|req| async move { result_to_response(web::articles::feed(req).await) });
        api.at("/articles/:slug")
            .get(|req| async move { result_to_response(web::articles::get_article(req).await) })
            .put(|req| async move { result_to_response(web::articles::update_article(req).await) })
            .delete(
                |req| async move { result_to_response(web::articles::delete_article(req).await) },
            );
        api.at("/articles/:slug/comments")
            .get(|req| async move { result_to_response(web::comments::get(req).await) })
            .post(|req| async move { result_to_response(web::comments::create(req).await) });
        api.at("/articles/:slug/comments/:id")
            .delete(|req| async move { result_to_response(web::comments::delete(req).await) });
        api.at("/articles/:slug/favorite")
            .post(|req| async move { result_to_response(web::articles::favorite(req).await) })
            .delete(|req| async move { result_to_response(web::articles::unfavorite(req).await) });
    });
    app
}

pub fn add_middleware(mut app: Server<Repo>) -> Server<Repo> {
    app.middleware(tide::middleware::RequestLogger::new());
    app.middleware(crate::middleware::JwtMiddleware::new());
    app
}
