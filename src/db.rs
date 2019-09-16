use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use futures::compat::Compat01As03;
use futures_01::future::poll_fn;
use log::error;
use r2d2::{CustomizeConnection, Pool, PooledConnection};
use tokio_threadpool::blocking;

/// A database "repository", for running database workloads.
/// Manages a connection pool and running blocking tasks in a
/// way that does not block the tokio event loop.
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
    pub fn new(database_url: &str) -> Self {
        Self::from_pool_builder(database_url, r2d2::Builder::default())
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

    pub fn with_test_transactions(database_url: &str) -> Self {
        let customizer = TestConnectionCustomizer {};
        let builder = Pool::builder().connection_customizer(Box::new(customizer));
        Self::from_pool_builder(database_url, builder)
    }

    /// Runs the given closure in a way that is safe for blocking IO to the database.
    /// The closure will be passed a `Connection` from the pool to use.
    pub async fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce(PooledConnection<ConnectionManager<T>>) -> R
            + Send
            + std::marker::Unpin
            + 'static,
        T: Send + 'static,
    {
        let pool = self.connection_pool.clone();
        // `tokio_threadpool::blocking` returns a `Poll` compatible with "old style" futures.
        // `poll_fn` converts this into a future, then
        // `tokio::await` is used to convert the old style future to a `std::futures::Future`.
        // `f.take()` allows the borrow checker to be sure `f` is not moved into the inner closure
        // multiple times if `poll_fn` is called multple times.
        let mut f = Some(f);
        Compat01As03::new(poll_fn(|| {
            blocking(|| (f.take().unwrap())(pool.get().unwrap()))
                .map_err(|e| panic!("the threadpool shut down: {:?}", e))
        }))
        .await
        .expect("Error running async database task.")
    }
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
