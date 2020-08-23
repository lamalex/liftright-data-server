use std::env;
use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod user;
pub mod schema;
pub mod survey;
pub mod imurecords;
pub mod repetition;

#[derive(Debug)]
pub enum LiftrightError {
    EnvironmentError(dotenv::Error),
    DatabaseError(diesel::result::Error),
}

pub type DbConnection = PgConnection;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!("./migrations/");

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url).expect("Failed to establish connection pool");

    let conn: DbPooledConnection = pool.get().unwrap();
    embedded_migrations::run(&conn).expect("Failed to run database migrations");

    pool
}

fn create_pool(db_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}
