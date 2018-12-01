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

impl Repo {
    pub fn new() -> Self {
        Repo {
            connection_pool: connection_pool(),
        }
    }

    pub async fn run<F, T>(&self, f: F) -> Result<T, tokio::io::Error>
    where
        F: FnOnce(Connection) -> T + Send + std::marker::Unpin + 'static,
        T: Send + 'static,
    {
        let pool = self.connection_pool.clone();
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
