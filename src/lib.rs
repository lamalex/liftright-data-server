#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate uuid;
extern crate chrono;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod user;
pub mod repetition;

#[derive(Debug)]
pub enum LiftrightError {
    EnvironmentError(dotenv::Error),
    DatabaseError(diesel::result::Error)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}