#[macro_use]
extern crate diesel;

use diesel::PgConnection;

pub mod auth;
pub mod conduit;
pub mod configuration;
pub mod db;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod web;

type Repo = db::Repo<PgConnection>;
