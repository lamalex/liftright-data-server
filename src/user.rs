use mongodb::{bson, options::FindOneOptions, Collection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{session::Session, LiftrightError};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RtfbStatusQuery {
    device_id: Uuid,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RtfbStatusResult {
    rtfb_status: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub device_id: Uuid,
    pub rtfb: bool,
    pub sessions: Vec<Session>,
}

impl User {
    pub fn get_or_make_if_new(
        _collection: Collection,
        _device_id: &Uuid,
    ) -> Result<User, LiftrightError> {
        Err(LiftrightError::UnimplementedError)
    }

    pub async fn check_rtfb_status(
        collection: Collection,
        device_id: Uuid,
    ) -> Result<bool, LiftrightError> {
        let filter = bson::to_document(&RtfbStatusQuery { device_id })
            .map_err(LiftrightError::DbSerializationError)?;

        let _rtfb_status_projection = FindOneOptions::builder()
            .projection(Some(bson::doc! {
                "rtfb_status": 1
            }))
            .build();

        let doc = collection
            .find_one(filter, None)
            .await
            .map_err(LiftrightError::DbError)?;

        match doc {
            Some(doc) => {
                let result: RtfbStatusResult =
                    bson::from_document(doc).map_err(LiftrightError::DbDeserializationError)?;
                Ok(result.rtfb_status)
            }
            None => Ok(false),
        }
    }
}
