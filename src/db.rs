use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use futures_01::future::poll_fn;
use r2d2::{Pool, PooledConnection};
use std::env;
use tokio_threadpool::blocking;
use futures::compat::Compat01As03;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// A database "repository", for running database workloads.
/// Manages a connection pool and running blocking tasks in a
/// way that does not block the tokio event loop.
#[derive(Clone)]
pub struct Repo {
    connection_pool: ConnectionPool,
}

impl Repo {
    pub fn new() -> Self {
        Repo {
            connection_pool: connection_pool(),
        }
    }

    /// Creates a repo with a pool builder, allowing you to customize
    /// any connection pool configuration.
    ///
    /// ```rust
    /// # use diesel::sqlite::SqliteConnection;
    /// use r2d2::Pool;
    /// use core::time::Duration;
    ///
    /// type Repo = db::Repo<SqliteConnection>;
    /// let database_url = ":memory:";
    /// let repo = Repo::from_pool_builder(database_url,
    ///     Pool::builder()
    ///         .connection_timeout(Duration::from_secs(120))
    ///         .max_size(100)
    /// );
    /// ```
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

    /// Runs the given closure in a way that is safe for blocking IO to the database.
    /// The closure will be passed a `Connection` from the pool to use.
    pub async fn run<F, T>(&self, f: F) -> T
    where
        F: FnOnce(Connection) -> T + Send + std::marker::Unpin + 'static,
        T: Send + 'static,
    {
        let pool = self.connection_pool.clone();
        // `tokio_threadpool::blocking` returns a `Poll` compatible with "old style" futures.
        // `poll_fn` converts this into a future, then
        // `tokio::await` is used to convert the old style future to a `std::futures::Future`.
        // `f.take()` allows the borrow checker to be sure `f` is not moved into the inner closure
        // multiple times if `poll_fn` is called multple times.
        let mut f = Some(f);
        Compat01As03::new( poll_fn(|| blocking(|| (f.take().unwrap())(
            pool.get().unwrap()
        ))
        .map_err(|_| panic!("the threadpool shut down")))).await
        .expect("Error running async database task.")
    }
}

pub fn connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    configure_pool(manager)
}

#[cfg(test)]
fn configure_pool(manager: ConnectionManager<PgConnection>) -> ConnectionPool {
    use crate::test_helpers::TestConnectionCustomizer;
    let customizer = TestConnectionCustomizer {};

    Pool::builder()
        .connection_customizer(Box::new(customizer))
        .build(manager)
        .expect("could not initiate test db pool")
}

#[cfg(not(test))]
fn configure_pool(manager: ConnectionManager<PgConnection>) -> ConnectionPool {
    Pool::new(manager).expect("could not initiate db pool")
}
