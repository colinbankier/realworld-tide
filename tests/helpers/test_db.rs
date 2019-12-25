use diesel::PgConnection;
use log::error;
use r2d2::{CustomizeConnection, Pool};
use realworld_tide::configuration::Settings;
use realworld_tide::db::Repo;

pub fn get_repo() -> Repo<PgConnection> {
    let settings = Settings::new().expect("Failed to load configuration");
    let customizer = TestConnectionCustomizer {};
    let builder = Pool::builder().connection_customizer(Box::new(customizer));
    Repo::from_pool_builder(&settings.database.connection_string(), builder)
}

#[derive(Debug)]
pub struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: diesel::connection::Connection,
    E: std::error::Error + Sync + Send,
{
    fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        if let Err(e) = conn.begin_test_transaction() {
            error!("Error beginning test transaction: {}", e);
        }
        Ok(())
    }
}
