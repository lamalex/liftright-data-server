use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;
use warp::reject::Reject;

pub mod imurecords;
pub mod repetition;
pub mod survey;
pub mod user;
pub mod userquery;

#[derive(Debug)]
pub enum LiftrightError {
    UnimplementedError,
    ConversionError,
    DbError(mongodb::error::Error),
    DbSerializationError(mongodb::bson::ser::Error),
    DbDeserializationError(mongodb::bson::de::Error),
}

impl Reject for LiftrightError {}

/// Establishes a connection to mongo db.
/// Records are stored as a tree in the form
/// {
/// "_id" : ObjectId,
/// "device_id" : Uuid,
/// "rtfb_status": bool,
///	"repetitions" : [
///		{
///			"session_id" : Uuid,
///			"set_id" : Uuid,
///			"level" : String,
///			"exercise" : String,
///			"number" : i32,
///			"rom" : f64,
///			"duration" : f64,
///			"time" : i64
///		},
///     ...,
/// "imu_data": [
///     {
///         "session_id": Uuid,
///         "data": [
///             {
///                 "acc" : {
///                     "y" : f64,
///                     "z" : f64,
///                     "x" : f64,
///                     "time" : i64,
///                 },
///                 "gyro" : {
///                     "y" : f64,
///                     "z" : f64,
///                     "x" : f64,
///                     "time" : i64,
///                 }
///             },
///             ...,
///         ]
///     },
///     ...,
/// ]
/// }
pub async fn establish_db_connection() -> Result<Collection, LiftrightError> {
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
    let client = Client::with_options(client_options).map_err(LiftrightError::DbError)?;
    let db = client.database("liftright");
    let collection = db.collection("session_data");

    Ok(collection)
}
