use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;
use warp::reject::Reject;

pub mod imurecords;
pub mod repetition;
pub mod session;
pub mod set;
pub mod survey;
pub mod user;

#[derive(Debug)]
pub enum LiftrightError {
    UnimplementedError,
    JsonSerializationError,
    DbError(mongodb::error::Error),
    DbSerializationError(mongodb::bson::ser::Error),
    DbDeserializationError(mongodb::bson::de::Error),
}

impl Reject for LiftrightError {}

/// Establishes a connection to mongo db.
/// Records are stored as a tree in the form
/// {
///     device_id: Uuid,
///     rtfb: bool,
///     sessions: [
///         {
///             session_id: Uuid,
///             level: String
///             sets: [
///                 {
///                     set_id: Uuid,
///                     exercise: String
///                     repetitions: [
///                         {
///                             number: Int32,
///                             rom: Double,
///                             velocity: Double,
///                             duration: Double,
///                             rep_time: Int64,
///                         },
///                         ...,
///                     ]
///                 },
///                 ...,
///             ],
///             imu_records: [
///                 {
///                     acc: [{ x: Double, y: Double, z: Double, time: Int64 }, ..., ]
///                     gyro: [{ x: Double, y: Double, z: Double, time: Int64 }, ...,]
///                 },
///                 ...,
///             ]
///         },
///         ...
///     ],
///     surveys: [
///         {
///             submitted: DateTime<Utc>,
///             data: [
///                 { question: String, answer: String },
///                 ...
///             ]
///         }
///     ]
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
