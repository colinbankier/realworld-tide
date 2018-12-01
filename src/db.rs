use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use futures::future::poll_fn;
use std::env;
use tokio_threadpool::blocking;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Clone)]
pub struct Repo {
    connection_pool: ConnectionPool,
}

/// A database "repository", for running database workloads.
/// Manages a connection pool and running blocking tasks in a
/// way that does not block the tokio event loop.
impl Repo {
    pub fn new() -> Self {
        Repo {
            connection_pool: connection_pool(),
        }
    }

    /// Runs the given closure in a way that is safe for blocking IO to the database.
    /// The closure will passed a `Connection` from the pool to use.
    pub async fn run<F, T>(&self, f: F) -> Result<T, tokio::io::Error>
    where
        F: FnOnce(Connection) -> T + Send + std::marker::Unpin + 'static,
        T: Send + 'static,
    {
        let pool = self.connection_pool.clone();
        // `tokio_threadpool::blocking` returns a `Poll` compatible with "old style" futures.
        // `poll_fn` converts this into a future, then
        // `tokio::await` is used to convert the old style future to a `std::futures::Future`.
        //
        // Currently fails compilation with:
        // |     pub async fn run<F, T>(&self, f: F) -> Result<T, tokio::io::Error>
        // |                                   - captured outer variable
        //
        // |             || blocking(|| f(pool.get().unwrap())).map_err(|_| panic!("the threadpool shut down"))
        // |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of captured variable in an `FnMut` closure
        tokio::await!(poll_fn(
            || blocking(|| f(pool.get().unwrap())).map_err(|_| panic!("the threadpool shut down"))
        ))
    }
}

pub fn connection_pool() -> ConnectionPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::new(manager).unwrap()
}
