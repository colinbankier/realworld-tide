#![feature(async_await, futures_api)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate http;

pub mod models;
pub mod schema;

use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use http::status::StatusCode;
use std::env;
use tide::{self, body, head, App, AppData};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

async fn list_users() -> Result<body::Json<Vec<User>>, StatusCode> {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.limit(5).load::<User>(&connection);

    results
        .map(|user_list| body::Json(user_list))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn main() {
    let mut app = tide::App::new(());
    app.at("/").get(list_users);
    app.serve("127.0.0.1:7878")
}
