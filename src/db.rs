use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use futures::future::poll_fn;
use std::env;
use tokio_threadpool::blocking;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

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

    /// Runs the given closure in a way that is safe for blocking IO to the database.
    /// The closure will be passed a `Connection` from the pool to use.
    pub async fn run<F, T>(&self, f: F) -> Result<T, tokio::io::Error>
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
        tokio::await!(poll_fn(|| blocking(|| (f.take().unwrap())(
            pool.get().unwrap()
        ))
        .map_err(|_| panic!("the threadpool shut down"))))
    }
}

pub fn connection_pool() -> ConnectionPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::new(manager).unwrap()
}
