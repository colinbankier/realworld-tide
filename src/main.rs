#![feature(async_await, futures_api, await_macro, pin, arbitrary_self_types)]
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
extern crate futures;

mod conduit;
mod db;
mod models;
mod schema;

use crate::conduit::*;
use crate::db::Repo;
use tide::App;

fn main() {
    let mut app = App::new(Repo::new());
    app.at("/api/user").get(get_user);
    app.at("/api/articles").get(list_articles);
    app.serve("127.0.0.1:7878")
}
