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
use diesel::PgConnection;
use tide::Server;
use async_std::task::block_on;

type Repo = db::Repo<PgConnection>;

pub fn set_routes(mut app: Server<Repo>) -> Server<Repo> {
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
