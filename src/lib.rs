#[macro_use]
extern crate diesel;

use crate::domain::repositories::Repository;
use diesel::PgConnection;

pub mod conduit;
pub mod configuration;
pub mod db;
pub mod domain;
pub mod web;

pub struct Context<R: 'static + Repository + Sync + Send> {
    pub repository: R,
}
type Repo = db::Repo<PgConnection>;
