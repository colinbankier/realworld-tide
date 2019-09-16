#![feature(arbitrary_self_types, async_closure)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

mod auth;
mod conduit;
mod db;
mod middleware;
mod models;
mod schema;
mod web;
mod configuration;

#[cfg(test)]
mod test_helpers;

use diesel::PgConnection;
use tide::App;
use crate::configuration::Settings;

type Repo = db::Repo<PgConnection>;

pub fn set_routes(mut app: App<Repo>) -> App<Repo> {
    app.at("/api").nest(|api| {
        api.at("/user").get(web::users::get_user);
        api.at("/user").put(web::users::update_user);
        api.at("/users").post(web::users::register);
        api.at("/users/login").post(web::users::login);
        api.at("/articles").get(web::articles::list_articles);
    });
    app
}

fn main() {
    let settings = Settings::new().expect("Failed to load configuration");
    env_logger::init();

    let state = Repo::new(&settings.database.connection_string());
    let mut app = App::with_state(state);
    app = set_routes(app);
    app.run(format!("{}:{}", settings.application.host, settings.application.port)).unwrap();
}
