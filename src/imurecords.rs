use crate::{
    session::Session,
    traits::{BucketUpdate, IdBucketPattern, Sanitize},
    LrdsError, LrdsResult,
};
use mongodb;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

const BUCKET_IMU_RECORD_LIMIT: i32 = 1000;

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
    session: Session,
    data: Vec<ImuRecordPair>,
}

enum Selector<'a> {
    Query(&'a ImuRecordSet),
    Update(&'a ImuRecordSet),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ImuRecordSetQuery {
    id: bson::Bson,
    device_id: Uuid,
    session_id: Uuid,
    imu_record_count: bson::Bson,
}

impl From<&ImuRecordSet> for ImuRecordSetQuery {
    fn from(imu_set: &ImuRecordSet) -> Self {
        ImuRecordSetQuery {
            id: bson::Bson::to_bucket_selector(imu_set.session.device_id),
            device_id: imu_set.session.device_id,
            session_id: imu_set.session.session_id,
            imu_record_count: bson::bson!({ "$lt": BUCKET_IMU_RECORD_LIMIT }),
        }
    }
}

struct ImuRecordSetUpdate<'a> {
    on_field: &'a str,
    with_value: &'a [ImuRecordPair],
    increment_field: &'a str,
    id_prefix: String,
}

impl From<&ImuRecordSet> for ImuRecordSetUpdate<'_> {
    fn from(imu_set: &ImuRecordSet) -> Self {
        ImuRecordSetUpdate {
            on_field: "imu_data",
            with_value: &imu_set.data[..],
            increment_field: "imu_record_count",
            id_prefix: imu_set.session.device_id.sanitize(),
        }
    }
}

impl TryFrom<&ImuRecordSetUpdate<'_>> for bson::Document {
    type Error = LrdsError;

    fn try_from(update_values: &ImuRecordSetUpdate) -> LrdsResult<Self> {
        Ok(bson::Document::to_bucket_update(
            update_values.on_field,
            &update_values.with_value,
            update_values.increment_field,
            update_values.with_value.len() as i32,
            &update_values.id_prefix,
        ))
    }
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
    pub async fn insert(self, _collection: mongodb::Collection) -> LrdsResult<()> {
        let query = bson::Document::try_from(Selector::Query(&self))?;
        let update = bson::Document::try_from(Selector::Update(&self))?;

        Ok(())
    }
}
