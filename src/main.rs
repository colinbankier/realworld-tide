use async_std::task::block_on;
use realworld_tide::conduit::articles_repository::Repository;
use realworld_tide::configuration::Settings;
use realworld_tide::db::Repo;
use realworld_tide::web::get_app;

fn main() -> Result<(), std::io::Error> {
    let settings = Settings::new().expect("Failed to load configuration");
    env_logger::init();

    let state = Repository(Repo::new(&settings.database.connection_string()));
    let app = get_app(state);
    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );

    block_on(async {
        app.listen(address).await?;
        Ok(())
    })
}
