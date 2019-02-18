#![feature(async_await, futures_api, await_macro, arbitrary_self_types)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate http;
extern crate tokio_threadpool;
#[macro_use]
extern crate tokio;
extern crate env_logger;
extern crate futures as futures01;
#[macro_use]
extern crate log;

#[cfg(test)]
extern crate fake;

mod auth;
mod endpoints;
mod conduit;
mod db;
mod extractors;
mod models;
mod schema;

#[cfg(test)]
mod test_helpers;

use crate::endpoints::*;
use crate::db::Repo;
use tide::App;
use dotenv::dotenv;


fn main() {
    dotenv().ok();
    env_logger::init();
    let mut app = App::new(Repo::new());
    app.at("/api").nest(|api| {
        api.at("/user").get(get_user);
        api.at("/user").put(update_user);
        api.at("/users").post(register);
        api.at("/users/login").post(login);
        api.at("/articles").get(list_articles);
    });
    app.serve()
}
