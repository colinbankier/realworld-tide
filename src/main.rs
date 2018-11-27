#![feature(async_await, futures_api)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate http;

mod conduit;
mod db;
mod models;
mod schema;

use crate::conduit::*;
use tide::App;

fn main() {
    let mut app = App::new(db::connection_pool());
    app.at("/api/user").get(get_user);
    app.at("/api/articles").get(list_articles);
    app.serve("127.0.0.1:7878")
}
