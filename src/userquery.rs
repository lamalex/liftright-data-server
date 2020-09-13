use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use uuid::Uuid;

use crate::LiftrightError;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserQuery {
    device_id: Uuid,
}

impl UserQuery {
    pub fn new(device_id: Uuid) -> Self {
        Self { device_id }
    }
}

impl TryInto<bson::Document> for UserQuery {
    type Error = LiftrightError;

    fn try_into(self) -> Result<bson::Document, Self::Error> {
        bson::to_document(&self).map_err(LiftrightError::DbSerializationError)
    }
}
