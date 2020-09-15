use mongodb;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    query_selector::{ImuRecordSetQuery, ImuRecordSetUpdate},
    session::Session,
    LrdsError, LrdsResult,
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImuRecord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub time: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImuRecordPair {
    pub acc: ImuRecord,
    pub gyro: ImuRecord,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImuRecordSet {
    #[serde(flatten)]
    pub session: Session,
    pub data: Vec<ImuRecordPair>,
}

enum Selector<'a> {
    Query(&'a ImuRecordSet),
    Update(&'a ImuRecordSet),
}

impl<'a> TryFrom<Selector<'a>> for bson::Document {
    type Error = LrdsError;
    fn try_from(var: Selector) -> LrdsResult<bson::Document> {
        use Selector::*;
        match var {
            Query(imu_set) => bson::to_document(&ImuRecordSetQuery::from(imu_set))
                .map_err(LrdsError::DbSerializationError),
            Update(imu_set) => bson::Document::try_from(&ImuRecordSetUpdate::from(imu_set)),
        }
    }
}

impl ImuRecordSet {
    pub async fn insert(self, collection: mongodb::Collection) -> LrdsResult<()> {
        let query = bson::Document::try_from(Selector::Query(&self))?;
        let update = bson::Document::try_from(Selector::Update(&self))?;
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        collection
            .update_one(query, update, options)
            .await
            .map_err(LrdsError::DbError)
            .map(|_| ())
    }
}
