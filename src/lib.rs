use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;
use warp::reject::Reject;

pub mod imurecords;
pub mod json_api;
pub mod query_selector;
pub mod repetition;
pub mod session;
pub mod set;
pub mod survey;
pub mod traits;
pub mod user;

#[derive(Debug)]
pub enum LrdsError {
    ConversionError,
    UnimplementedError,
    DbError(mongodb::error::Error),
    ObjectIdError(mongodb::bson::oid::Error),
    DbSerializationError(mongodb::bson::ser::Error),
    DbDeserializationError(mongodb::bson::de::Error),
}

impl Reject for LrdsError {}
type LrdsResult<T> = Result<T, LrdsError>;

/// Establishes a connection to mongo db.
/// Records are stored as a tree in the form
/// TODO
pub async fn establish_db_connection() -> Result<Collection, LrdsError> {
    dotenv().ok();

    let database_url = env::var("MONGO_DATABASE_ADDR").expect("MONGO_DATABASE_ADDR must be set");
    let database_port = env::var("MONGO_DATABASE_PORT")
        .unwrap_or("27017".to_string())
        .parse::<u16>()
        .expect("MONGO_DATABASE_PORT must be an unsigned 16-bit integer");

    let client_options = ClientOptions::builder()
        .hosts(vec![mongodb::options::StreamAddress {
            hostname: database_url.to_string(),
            port: Some(database_port),
        }])
        .app_name(Some("LiftRight".to_string()))
        .build();
    let client = Client::with_options(client_options).map_err(LrdsError::DbError)?;
    let db = client.database("liftright");
    let collection = db.collection("session_data");

    Ok(collection)
}
