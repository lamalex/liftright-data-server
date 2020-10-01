use crate::{LrdsError, LrdsResult};
use mongodb;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct User {
    device_id: Uuid,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
struct RtfbStatusFilter {
    device_id: Uuid,
    rtfb_status: bool,
}

impl From<User> for RtfbStatusFilter {
    fn from(user: User) -> Self {
        RtfbStatusFilter {
            device_id: user.device_id,
            rtfb_status: true,
        }
    }
}

impl TryFrom<User> for bson::Document {
    type Error = LrdsError;
    fn try_from(user: User) -> LrdsResult<Self> {
        bson::to_document(&RtfbStatusFilter::from(user)).map_err(LrdsError::DbSerializationError)
    }
}

impl User {
    pub fn new(device_id: Uuid) -> Self {
        User { device_id }
    }

    pub async fn check_rtfb_status(self, collection: mongodb::Collection) -> LrdsResult<bool> {
        let filter = bson::Document::try_from(self)?;
        Ok(collection
            .find_one(filter, None)
            .await
            .map_err(LrdsError::DbError)?
            .is_some())
    }
}
