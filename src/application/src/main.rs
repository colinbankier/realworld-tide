use async_std::task::block_on;
use db::{connection::Repo, Repository};
use realworld_application::configuration::Settings;
use std::path::PathBuf;
use web::get_app;

fn main() -> Result<(), std::io::Error> {
    let settings = Settings::new(PathBuf::default()).expect("Failed to load configuration");
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
