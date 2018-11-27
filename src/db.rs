use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connection_pool() -> ConnectionPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::new(manager).unwrap()
}
