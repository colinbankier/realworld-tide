use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use r2d2::{Pool, PooledConnection};

/// A database "repository", for running database workloads.
#[derive(Clone)]
pub struct Repo<T>
where
    T: Connection + 'static,
{
    connection_pool: Pool<ConnectionManager<T>>,
}

impl<T> Repo<T>
where
    T: Connection + 'static,
{
    /// Creates a repo using default configuration for the underlying connection pool.
    pub fn new(database_url: &str) -> Self {
        Self::from_pool_builder(database_url, r2d2::Builder::default())
    }

    /// Creates a repo with a pool builder, allowing you to customize
    /// any connection pool configuration.
    pub fn from_pool_builder(
        database_url: &str,
        builder: r2d2::Builder<ConnectionManager<T>>,
    ) -> Self {
        let manager = ConnectionManager::new(database_url);
        let connection_pool = builder
            .build(manager)
            .expect("could not initiate test db pool");
        Repo { connection_pool }
    }

    pub fn conn(&self) -> PooledConnection<ConnectionManager<T>> {
        self.connection_pool.get().unwrap()
    }
}
