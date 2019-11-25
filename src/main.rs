#[macro_use]
extern crate diesel;

mod auth;
mod conduit;
mod db;
mod middleware;
mod models;
mod schema;
mod web;

#[cfg(test)]
mod test_helpers;

use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use tide::App;

type Repo = db::Repo<PgConnection>;

pub fn set_routes(mut app: App<Repo>) -> App<Repo> {
    app.at("/api").nest(|api| {
        api.at("/user").get(web::users::get_user);
        api.at("/user").put(web::users::update_user);
        api.at("/users").post(web::users::register);
        api.at("/users/login").post(web::users::login);
        api.at("/articles").get(web::articles::list_articles);
        api.at("/articles/:slug").get(web::articles::get_article);
    });
    app
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut app = App::with_state(Repo::new(&database_url));
    app = set_routes(app);
    app.serve("127.0.0.1:8000").unwrap();
}
