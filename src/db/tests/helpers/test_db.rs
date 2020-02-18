use application::configuration::Settings;
use diesel::connection::SimpleConnection;
use log::error;
use r2d2::{CustomizeConnection, Pool};
use realworld_db::{Repo, Repository};
use std::path::PathBuf;

pub fn get_repo() -> Repository {
    let settings = Settings::new(PathBuf::from("../../")).expect("Failed to load configuration");
    Repository(Repo::new(&settings.database.connection_string()))
}

/// The returned repository executes all queries in a SQL transaction,
/// which is never committed (hence the DB state never changes for other observers).
///
/// Its useful to speed up tests where all DB interactions are executed
/// from the same instance of Repo<PgConnection>
/// (e.g. no need to drop rows at the end of the test).
pub fn get_test_repo() -> Repo {
    let settings = Settings::new(PathBuf::from("../../")).expect("Failed to load configuration");
    let customizer = TestConnectionCustomizer {};
    let builder = Pool::builder().connection_customizer(Box::new(customizer));
    Repo::from_pool_builder(&settings.database.connection_string(), builder)
}

/// Delete all rows in all the tables in the database.
pub fn clean_db(repo: &Repository) {
    repo.0
        .conn()
        .batch_execute("DELETE FROM users; DELETE FROM articles;")
        .expect("Failed to clean database")
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
