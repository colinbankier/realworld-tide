#![feature(async_await, futures_api, await_macro, arbitrary_self_types)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

mod auth;
mod conduit;
mod db;
mod extractors;
mod models;
mod query_string;
mod schema;

#[cfg(test)]
mod test_helpers;
mod web;

use crate::db::Repo;

use dotenv::dotenv;
use tide::App;

pub fn application() -> App<Repo> {
    dotenv().ok();
    env_logger::init();
    let app = App::new(Repo::new());
    set_routes(app)
}

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
