use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn connection() -> DbPool {
    dotenv().ok();

    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    init_pool(&postgres_url).unwrap_or_else(|_| panic!("Error connecting to {}", postgres_url))
}
