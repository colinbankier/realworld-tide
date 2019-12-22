#[macro_use]
extern crate diesel;

mod auth;
mod conduit;
mod configuration;
mod db;
mod middleware;
mod models;
mod schema;
mod web;

#[cfg(test)]
mod test_helpers;

use crate::configuration::Settings;
use async_std::task::block_on;
use diesel::PgConnection;
use tide::{IntoResponse, Request, Response, Server};

type Repo = db::Repo<PgConnection>;

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

pub fn set_routes(mut app: Server<Repo>) -> Server<Repo> {
    app.at("/api").nest(|api| {
        api.at("/user")
            .get(|mut req: tide::Request<Repo>| async move {
                result_to_response(web::users::get_user(req).await)
            });
        api.at("/user")
            .put(|mut req: tide::Request<Repo>| async move {
                result_to_response(web::users::update_user(req).await)
            });
        api.at("/users")
            .post(|mut req: tide::Request<Repo>| async move {
                result_to_response(web::users::register(req).await)
            });
        api.at("/users/login")
            .post(|mut req: tide::Request<Repo>| async move {
                result_to_response(web::users::login(req).await)
            });
        api.at("/articles")
            .get(|mut req: tide::Request<Repo>| async move {
                result_to_response(web::articles::list_articles(req).await)
            });
        api.at("/articles/:slug")
            .get(|mut req: tide::Request<Repo>| async move {
                result_to_response(web::articles::get_article(req).await)
            });
    });
    app
}

fn main() -> Result<(), std::io::Error> {
    let settings = Settings::new().expect("Failed to load configuration");
    env_logger::init();

    let state = Repo::new(&settings.database.connection_string());
    let mut app = Server::with_state(state);
    app = set_routes(app);
    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );

    block_on(async {
        app.listen(address).await?;
        Ok(())
    })
}
